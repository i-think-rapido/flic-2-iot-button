
mod enums;
mod events;
mod commands;
mod client;

use std::error::Error;

use client::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let event = event_handler(|event| { println!("ping response: {:?}", event); });
    let event2 = event_handler(|event| { println!("ping response: {:?}", event); });

    let _client = FlicClient::new("127.0.0.1:5551").await?
    .register_event_handler(event)
    .register_event_handler(event2)
    ;

    Ok(())
}
