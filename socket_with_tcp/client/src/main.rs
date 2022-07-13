use client::SocketClient;

// cargo run --package client
fn main() -> std::io::Result<()> {
    SocketClient::switch_on()?;
    let status = SocketClient::get_status()?;
    println!("{}", status);
    SocketClient::switch_off()?;
    let status = SocketClient::get_status()?;
    println!("{}", status);
    Ok(())
}
