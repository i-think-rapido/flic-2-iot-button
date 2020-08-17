

use bytes::BufMut;
use tokio::net::TcpStream;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::io::*;
use tokio::sync::Mutex;

use std::net::Shutdown;
//use std::sync::Arc;
//use std::time::Duration;

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
    pub async fn register_event_handler(mut self, event: EventClosureMutex) -> Self {
        self.map.lock().await.push(event);
        self
    }
    pub async fn listen(&self) {
        let mut buffer = vec![];
//        if let Some(size) = self.reader.lock().await.peek(&mut buffer).await.ok() {
  //          if size > 0 {
                if let Some(_) = self.reader.lock().await.read_buf(&mut buffer).await.ok() {
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
    //            }
      //      }
        }
    }
    pub async fn is_running(&self) -> bool {
        return *self.is_running.lock().await
    }
    pub async fn stop(&self) {
        *self.is_running.lock().await = false;
        //self.reader.lock().await.shutdown();
        //self.writer.lock().await.shutdown();
    }

    pub async fn submit(&self, cmd: Command) {
        let mut writer = self.writer.lock().await;
        for b in self.command_mapper.lock().await.map(cmd) {
            writer.write_u8(b).await;
            println!("{:?}", b);
        }
    }
}

