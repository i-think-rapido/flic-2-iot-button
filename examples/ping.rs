

use std::error::Error;
use std::time::Duration;
use std::sync::Arc;

use flicbtn::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let event = event_handler(|event| { println!("ping response: {:?}", event); });
    let event2 = event_handler(|event| { println!("ping response: {:?}", event); });

    let client = FlicClient::new("127.0.0.1:5551").await?
        .register_event_handler(event).await
        .register_event_handler(event2).await
    ;
    let client1 = Arc::new(client);
    let client2 = client1.clone();

    let cmd = tokio::spawn(async move {
        client1.submit(Command::GetInfo).await;
        tokio::time::delay_for(Duration::from_secs(3)).await;
        client1.submit(Command::GetInfo).await;
        tokio::time::delay_for(Duration::from_secs(3)).await;
        client1.stop().await;
    });
    let lst = tokio::spawn(async move {
        client2.listen().await;
        println!("stop");
    });

    lst.await?;
    cmd.await?;

    Ok(())
}
