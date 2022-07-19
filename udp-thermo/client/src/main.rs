use std::io::Write;
use std::net::{SocketAddr, TcpStream, UdpSocket};
use std::{io, thread};
use std::time::Duration;

fn main() -> io::Result<()> {
    let addr = negotiate_addr()?;
    let socket = UdpSocket::bind(addr)?;
    socket.set_read_timeout(Some(Duration::from_secs(1)))?;
    loop {
        let mut buf = [0; 8];
        if let Err(err) = socket.recv_from(&mut buf) {
            println!("Failed to read data: {}", err);
        }
        let temp = f64::from_be_bytes(buf);
        println!("Received temp: {}", temp);
        thread::sleep(Duration::from_secs(1));
    }
}

fn negotiate_addr() -> io::Result<SocketAddr> {
    let mut stream = TcpStream::connect("127.0.0.1:7000")?;
    let cmd = "init".as_bytes();
    stream.write_all(&cmd)?;
    Ok(stream.local_addr()?)
}
