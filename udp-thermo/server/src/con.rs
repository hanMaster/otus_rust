use crate::pool::ClientPool;
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};

#[derive(Default)]
pub struct Connector;

impl Connector {
    pub async fn bind<Addrs>(&self, addrs: Addrs) -> io::Result<TcpListener>
    where
        Addrs: ToSocketAddrs,
    {
        let tcp = TcpListener::bind(addrs).await?;
        Ok(tcp)
    }

    pub async fn handle_client(
        &mut self,
        mut stream: TcpStream,
        pool: Arc<ClientPool>,
    ) -> io::Result<()> {
        let addr = stream.peer_addr().expect("Failed to get peer address");
        println!("Peer connected: {:?}", addr);
        let mut buf = [0; 4];
        stream.read_exact(&mut buf).await?;
        let decoded = buf.as_slice();
        match decoded {
            b"init" => {
                println!("Connection requested");
                pool.store_client(addr);
            }
            b"done" => {
                println!("Terminate connection requested");
                stream.read_exact(&mut buf).await?;
                let len = u32::from_be_bytes(buf);
                let mut buf = vec![0; len as _];
                stream.read_exact(&mut buf).await?;
                let addr = String::from_utf8(buf).expect("Failed to parse data");
                pool.remove_client(addr.parse::<SocketAddr>().expect("Failed to create addr"));
            }
            _ => println!("received: bad command"),
        }
        Ok(())
    }
}
