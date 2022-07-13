use client::SocketClient;

// cargo run --package client
fn main() -> std::io::Result<()> {
    let status = SocketClient::get_status()?;
    println!("{}", status);
    Ok(())
}
