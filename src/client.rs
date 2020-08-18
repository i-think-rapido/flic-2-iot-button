

use futures::future::poll_fn;
use futures::task::Poll;
use tokio::net::TcpStream;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::io::*;
use tokio::sync::Mutex;

use super::events::Event;
use super::events::stream_mapper::*;
use super::commands::stream_mapper::CommandToByteMapper;
use super::commands::Command;

type EventClosure = dyn FnMut(Event) + Sync + Send + 'static;
type EventClosureMutex = Box<EventClosure>;

pub fn event_handler<F>(f: F) -> EventClosureMutex
        where F: FnMut(Event) + Sync + Send + 'static
    {
        Box::new(f)
    }

pub struct FlicClient {
    reader: Mutex<OwnedReadHalf>,
    writer: Mutex<OwnedWriteHalf>,
    is_running: Mutex<bool>,
    command_mapper: Mutex<CommandToByteMapper>,
    event_mapper: Mutex<ByteToEventMapper>,
    map: Mutex<Vec<EventClosureMutex>>,
}

impl FlicClient {
    pub async fn new(conn: &str) -> Result<FlicClient> {
        match TcpStream::connect(conn).await {

            Ok(stream) => {
                let (reader, writer) = stream.into_split();
                Ok(FlicClient{
                    reader: Mutex::new(reader),
                    writer: Mutex::new(writer),
                    is_running: Mutex::new(true),
                    command_mapper: Mutex::new(CommandToByteMapper::new()),
                    event_mapper: Mutex::new(ByteToEventMapper::new()),
                    map: Mutex::new(vec![]),
                })
            }
            Err(err) => Err(err)
        }
        
    }
    pub async fn register_event_handler(self, event: EventClosureMutex) -> Self {
        self.map.lock().await.push(event);
        self
    }
    pub async fn listen(&self) {
        while *self.is_running.lock().await {
            let mut reader = self.reader.lock().await;
            if let Ok(size) = poll_fn(|cx| {
                let mut buf = [0; 1];
                match reader.poll_peek(cx, &mut buf) {
                    Poll::Pending => Poll::Ready(Ok(0_usize)),
                    Poll::Ready(all) => Poll::Ready(all),
                }
            }).await{
                if size > 0 {
                    let mut buffer = vec![];
                    if let Some(_) = reader.read_buf(&mut buffer).await.ok() {
                        for b in buffer.iter() {
                            match self.event_mapper.lock().await.map(*b) {
                                EventResult::Some(Event::NoOp) => {}
                                EventResult::Some(event) => {
                                    let mut map = self.map.lock().await;
                                    for ref mut f in &mut *map {
                                        f(event.clone());
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }
    pub async fn stop(&self) {
        *self.is_running.lock().await = false;
    }

    pub async fn submit(&self, cmd: Command) {
        let mut writer = self.writer.lock().await;
        for b in self.command_mapper.lock().await.map(cmd) {
            let _ = writer.write_u8(b).await;
        }
    }
}

