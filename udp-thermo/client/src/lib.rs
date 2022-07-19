use std::io;
use std::io::Write;
use std::net::{SocketAddr, TcpStream, UdpSocket};
use std::time::Duration;

const SRV_ADDRESS: &str = "127.0.0.1:7000";

pub struct ThermoClient {
    addr: SocketAddr,
    socket: UdpSocket,
}

impl ThermoClient {
    pub fn new() -> Self {
        let addr = ThermoClient::negotiate_addr().expect("Failed to init address");
        let socket = UdpSocket::bind(addr).expect("Failed create socket");
        socket
            .set_read_timeout(Some(Duration::from_secs(1)))
            .expect("Failed to setup socket");
        Self { addr, socket }
    }

    fn negotiate_addr() -> io::Result<SocketAddr> {
        let mut stream = TcpStream::connect(SRV_ADDRESS)?;
        let cmd = "init".as_bytes();
        stream.write_all(cmd)?;
        stream.local_addr()
    }

    pub fn close_connection(&self) -> io::Result<()> {
        let cmd = "done".as_bytes();
        let str = self.addr.to_string();
        let bytes = str.as_bytes();
        let length = bytes.len() as u32;
        let length_bytes = length.to_be_bytes();
        let mut stream = TcpStream::connect(SRV_ADDRESS)?;
        stream.write_all(cmd)?;
        stream.write_all(&length_bytes)?;
        stream.write_all(bytes)?;
        Ok(())
    }

    pub fn read_temperature(&self) -> io::Result<f64> {
        let mut buf = [0; 8];
        self.socket.recv_from(&mut buf)?;
        Ok(f64::from_be_bytes(buf))
    }
}

impl Default for ThermoClient {
    fn default() -> Self {
        ThermoClient::new()
    }
}
