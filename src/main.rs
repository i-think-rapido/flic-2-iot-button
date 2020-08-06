
mod enums;
mod events;
mod client;

use client::*;

fn main() {

    let event = event_handler(|event| { println!("ping response: {:?}", event); });
    let event2 = event_handler(|event| { println!("ping response: {:?}", event); });

    let _client = FlicClient::new()
    .register_event_handler(event)
    .register_event_handler(event2)
    ;
}
