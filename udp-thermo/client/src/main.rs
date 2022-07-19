use client::ThermoClient;
use std::time::Duration;
use std::{io, thread};

fn main() -> io::Result<()> {
    let tc = ThermoClient::default();

    for _ in 0..10 {
        match tc.read_temperature() {
            Ok(temp) => println!("Received temp: {:.2}", temp),
            Err(err) => eprintln!("Failed to read temperature: {}", err),
        }
        thread::sleep(Duration::from_secs(1));
    }
    tc.close_connection()?;
    Ok(())
}
