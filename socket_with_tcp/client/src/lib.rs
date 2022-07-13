use std::io::{Read, Write};
use std::net::TcpStream;
use socket::cmd::Commands;

pub struct SocketClient{}

impl SocketClient {
    pub fn get_status() -> std::io::Result<String> {
        let mut stream = TcpStream::connect("127.0.0.1:7000")?;
        stream.write(&Commands::get_status_cmd())?;
        let mut buf: [u8;8] = [0;8];
        stream.read_exact(&mut buf)?;
        Ok(String::from_utf8(Vec::try_from(buf).unwrap()).unwrap())
    }
}