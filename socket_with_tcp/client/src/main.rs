use std::io::Write;
use std::net::TcpStream;

// cargo run --package client
fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:7000")?;
    stream.write(b"Hello world")?;
    Ok(())
}
