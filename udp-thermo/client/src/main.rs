use client::ThermoClient;
use std::time::Duration;
use std::{io, thread};

#[tokio::main]
async fn main() -> io::Result<()> {
    let tc = ThermoClient::new().await;

    for _ in 0..10 {
        match tc.read_temperature().await {
            Ok(temp) => println!("Received temp: {:.2}", temp),
            Err(err) => eprintln!("Failed to read temperature: {}", err),
        }
        thread::sleep(Duration::from_secs(1));
    }
    tc.close_connection().await?;
    Ok(())
}
