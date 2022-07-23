use client::SocketClient;

// cargo run --package client
#[tokio::main]
async fn main() -> std::io::Result<()> {
    SocketClient::switch_on().await?;
    let status = SocketClient::get_status().await?;
    println!("{}", status);
    SocketClient::switch_off().await?;
    let status = SocketClient::get_status().await?;
    println!("{}", status);
    Ok(())
}
