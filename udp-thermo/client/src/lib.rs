use std::io;
use std::net::SocketAddr;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpStream, UdpSocket};

const SRV_ADDRESS: &str = "127.0.0.1:7000";

pub struct ThermoClient {
    addr: SocketAddr,
    socket: UdpSocket,
}

impl ThermoClient {
    pub async fn new() -> Self {
        let addr = ThermoClient::negotiate_addr()
            .await
            .expect("Failed to init address");
        let socket = UdpSocket::bind(addr).await.expect("Failed create socket");
        Self { addr, socket }
    }

    async fn negotiate_addr() -> io::Result<SocketAddr> {
        let mut stream = TcpStream::connect(SRV_ADDRESS).await?;
        let cmd = "init".as_bytes();
        stream.write_all(cmd).await?;
        stream.local_addr()
    }

    pub async fn close_connection(&self) -> io::Result<()> {
        let cmd = "done".as_bytes();
        let str = self.addr.to_string();
        let bytes = str.as_bytes();
        let length = bytes.len() as u32;
        let length_bytes = length.to_be_bytes();
        let mut stream = TcpStream::connect(SRV_ADDRESS).await?;
        stream.write_all(cmd).await?;
        stream.write_all(&length_bytes).await?;
        stream.write_all(bytes).await?;
        Ok(())
    }

    pub async fn read_temperature(&self) -> io::Result<f64> {
        let mut buf = [0; 8];
        self.socket.recv_from(&mut buf).await?;
        Ok(f64::from_be_bytes(buf))
    }
}
