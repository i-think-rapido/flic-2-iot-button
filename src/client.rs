
use std::sync::Arc;

use tokio::io::*;
use tokio::net::TcpStream;

use super::events::*;
use super::commands::Command;

pub fn event_handler<F>(f: F) -> Box<dyn FnMut(Event)>
        where F: FnMut(Event) + 'static
    {
        Box::new(f)
    }

pub struct FlicClient {
    reader: Arc<TcpStream>,
    writer: Arc<TcpStream>,
    map: Vec<Box<dyn FnMut(Event)>>,
    is_running: bool,
}

impl FlicClient {
    pub async fn new(conn: &str) -> Result<FlicClient> {
        match TcpStream::connect(conn).await {
            Ok(stream) => {
                let reader = Arc::new(stream);
                let writer = reader.clone();
    
                Ok(FlicClient{
                    reader,
                    writer,
                    map: vec![],
                    is_running: true,
                })
            }
            Err(err) => Err(err)
        }
        
    }
    pub fn register_event_handler(mut self, event: Box<dyn FnMut(Event)>) -> Self {
        self.map.push(event);
        self
    }
    pub async fn listen(&mut self) {
        while self.is_running {
            if let Some(mut r) = Arc::get_mut(&mut self.reader) {
                if let Ok(value) = r.read_u8().await {
                    for ref mut f in self.map.as_mut_slice() {
                        f(Event::read_event(value, &mut r));
                    }
                }
            }
        }
    }
    pub fn stop(&mut self) {
        self.is_running = false;
    }

    pub fn submit(&mut self, cmd: Command) -> Result<()> {
        if let Some(mut w) = Arc::get_mut(&mut self.writer) {
            cmd.write_command(&mut w)?;
        }
        Ok(())
    }
}

