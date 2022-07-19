use std::io::Write;
use std::net::{SocketAddr, TcpStream, UdpSocket};
use std::time::Duration;
use std::{io, thread};

fn main() -> io::Result<()> {
    let addr = negotiate_addr()?;
    let socket = UdpSocket::bind(addr)?;
    socket.set_read_timeout(Some(Duration::from_secs(1)))?;
    for _ in 0..10 {
        let mut buf = [0; 8];
        if let Err(err) = socket.recv_from(&mut buf) {
            println!("Failed to read data: {}", err);
        }
        let temp = f64::from_be_bytes(buf);
        println!("Received temp: {:.2}", temp);
        thread::sleep(Duration::from_secs(1));
    }
    close_connection(addr)?;
    Ok(())
}

fn negotiate_addr() -> io::Result<SocketAddr> {
    let mut stream = TcpStream::connect("127.0.0.1:7000")?;
    let cmd = "init".as_bytes();
    stream.write_all(cmd)?;
    stream.local_addr()
}

fn close_connection(addr: SocketAddr) -> io::Result<()> {
    let cmd = "done".as_bytes();
    let str = addr.to_string();
    let bytes = str.as_bytes();
    let length = bytes.len() as u32;
    let length_bytes = length.to_be_bytes();
    let mut stream = TcpStream::connect("127.0.0.1:7000")?;
    stream.write_all(cmd)?;
    stream.write_all(&length_bytes)?;
    stream.write_all(bytes)?;
    Ok(())
}
