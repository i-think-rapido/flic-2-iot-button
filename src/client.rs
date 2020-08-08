
use tokio::io::*;

use bytes::BufMut;
use tokio::net::TcpStream;

use std::net::Shutdown;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;

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
    stream: TcpStream,
    command_mapper: CommandToByteMapper,
    map: Vec<EventClosureMutex>,
    is_running: bool,
}

impl FlicClient {
    pub async fn new(conn: &str) -> Result<FlicClient> {
        match TcpStream::connect(conn).await {
            Ok(stream) => {
                println!("stream open");
                Ok(FlicClient{
                    stream,
                    command_mapper: CommandToByteMapper::new(),
                    map: vec![],
                    is_running: true,
                })
            }
            Err(err) => Err(err)
        }
        
    }
    pub fn register_event_handler(mut self, event: EventClosureMutex) -> Self {
        self.map.push(event);
        self
    }
    pub async fn listen(&mut self) {
        let mut mapper = ByteToEventMapper::new();
        let (mut reader, _writer) = self.stream.split();
        let mut buffer = vec![];
        while self.is_running {
            if let Some(size) = reader.read_buf(&mut buffer).await.ok() {
                for b in buffer.iter() {
                    match mapper.map(*b) {
                        EventResult::Some(Event::NoOp) => {}
                        EventResult::Some(event) => for ref mut f in &mut self.map {
                            f(event.clone());
                        }
                        _ => {}
                    }
                }

            }
        }
    }
    pub async fn stop(&mut self) {
        self.is_running = false;
        self.stream.shutdown(Shutdown::Both);
        println!("stopped");
    }

    pub async fn submit(&mut self, cmd: Command) {
        let (_reader, mut writer) = self.stream.split();
            for b in self.command_mapper.map(cmd) {
                writer.write_u8(b).await;
                println!("{:?}", b);
            }
    }
}

