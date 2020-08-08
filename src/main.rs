
mod enums;
mod events;
mod commands;
mod client;

use std::error::Error;
use std::time::Duration;
use std::sync::Arc;

use tokio::sync::Mutex;

use client::*;
use commands::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let event = event_handler(|event| { println!("ping response: {:?}", event); });
    let event2 = event_handler(|event| { println!("ping response: {:?}", event); });

    let mut client = FlicClient::new("127.0.0.1:5551").await?
    .register_event_handler(event)
    .register_event_handler(event2)
    ;
    
    client.submit(Command::GetInfo).await;
    client.listen().await;

    Ok(())
}
