use crate::pool::ClientPool;
use std::io;
use std::io::Read;
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::sync::Arc;

#[derive(Default)]
pub struct Connector;

impl Connector {
    pub fn bind<Addrs>(&self, addrs: Addrs) -> io::Result<TcpListener>
    where
        Addrs: ToSocketAddrs,
    {
        let tcp = TcpListener::bind(addrs)?;
        Ok(tcp)
    }

    pub fn handle_client(
        &mut self,
        mut stream: TcpStream,
        pool: Arc<ClientPool>,
    ) -> io::Result<()> {
        let addr = stream.peer_addr().expect("Failed to get peer address");
        println!("Peer connected: {:?}", addr);
        let mut buf = [0; 4];
        stream.read_exact(&mut buf)?;
        let decoded = buf.as_slice();
        match decoded {
            b"init" => {
                println!("Connection requested");
                pool.store_client(addr);
            }
            b"done" => {
                println!("Terminate connection requested");
                pool.remove_client(addr);
            }
            _ => println!("received: bad command"),
        }
        Ok(())
    }
}
