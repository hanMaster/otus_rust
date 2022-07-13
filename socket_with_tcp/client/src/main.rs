use std::io::Write;
use std::net::TcpStream;
use socket::cmd::Commands;

// cargo run --package client
fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:7000")?;
    stream.write(&Commands::get_status_cmd())?;
    // println!("send: {:?}", Commands::get_status_cmd());
    Ok(())
}
