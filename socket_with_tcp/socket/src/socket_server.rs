use std::io;
use std::io::Read;
use std::net::{TcpListener, TcpStream, ToSocketAddrs};

pub struct SocketServer {
    listener: TcpListener,
}

impl SocketServer {
    pub fn bind<Addrs>(addrs: Addrs) -> io::Result<TcpListener>
    where
        Addrs: ToSocketAddrs,
    {
        let tcp = TcpListener::bind(addrs)?;
        Ok(tcp)
    }

    pub fn handle_client(mut stream: TcpStream) -> io::Result<()> {
        let mut buf = String::new();
        stream.read_to_string(&mut buf)?;
        println!("{}", buf);
        Ok(())
    }
}
