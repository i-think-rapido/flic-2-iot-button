use std::io::*;
use std::sync::Arc;

use flicbtn::*;

#[tokio::main]
async fn main() -> Result<()> {
    let event = event_handler(|event| {
        println!("ping response: {:?}", event);
    });

    let client = FlicClient::new("127.0.0.1:5551")
        .await?
        .register_event_handler(event)
        .await;
    let client1 = Arc::new(client);
    let client2 = client1.clone();

    let button = "80:e4:da:76:fa:55";

    let mut scan_wizard_id = 0;
    let mut conn_id = 0;

    let cmd = tokio::spawn(async move {
        println!("===============================================");
        println!("*** Hello to the Flic2 Button Simple Client ***");
        println!("===============================================");
        client1.submit(Command::GetInfo).await;
        println!("");

        loop {
            show_commands();

            println!("");
            print!("-- Choose: ");

            let _ = stdout().flush();

            let mut input = String::new();
            stdin()
                .read_line(&mut input)
                .expect("Did not enter correct string!");
            let input = input.trim();

            match input.as_str() {
                "X" => break,
                "1" => {
                    println!("-- start scan wizard");
                    scan_wizard_id += 1;
                    client1
                        .submit(Command::CreateScanWizard { scan_wizard_id })
                        .await;
                }
                "2" => {
                    println!("-- cancel scan wizard");
                    client1
                        .submit(Command::CancelScanWizard { scan_wizard_id })
                        .await;
                    //scan_wizard_id -= 1;
                }
                "3" => {
                    println!("-- create connection channel");
                    conn_id += 1;
                    client1
                        .submit(Command::CreateConnectionChannel {
                            conn_id,
                            bd_addr: button.to_string(),
                            latency_mode: LatencyMode::NormalLatency,
                            auto_disconnect_time: 11111_i16,
                        })
                        .await;
                }
                "4" => {
                    println!("-- remove connection channel");
                    client1
                        .submit(Command::RemoveConnectionChannel { conn_id })
                        .await;
                    //conn_id -= 1;
                }
                "5" => {
                    println!("-- button info");
                    client1
                        .submit(Command::GetButtonInfo {
                            bd_addr: button.to_string(),
                        })
                        .await;
                }
                _ => {
                    println!("-- unknown command");
                }
            }

            println!("");
        }

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

fn show_commands() {
    println!("1) Start Scan Wizard");
    println!("2) Cancel Scan Wizard");
    println!("3) Create Connection Channel");
    println!("4) Remove Connection Channel");
    println!("5) Get Button Info");
    println!("X) End");
}
