use std::io;
use std::io::Read;
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use socket::crypt::Crypt;
use socket::Socket;

pub struct SocketServer {}

impl SocketServer {
    pub fn bind<Addrs>(addrs: Addrs) -> io::Result<TcpListener>
    where
        Addrs: ToSocketAddrs,
    {
        let tcp = TcpListener::bind(addrs)?;
        Ok(tcp)
    }

    pub fn handle_client(mut stream: TcpStream, socket: &mut Socket) -> io::Result<()> {
        let mut buf = [0;4];
        stream.read_exact(&mut buf)?;
        let decoded = buf.as_slice().decrypt();
        // println!("received: {}", String::from_utf8(decoded).unwrap());
        match decoded.as_slice() {
            b"cmd0" => {
                println!("received: {}", String::from_utf8(decoded).unwrap());
                socket.toggle_switch();
                let state = socket.get_state();
                println!("{}", state);
            },
            _ => println!("received: bad command")
        }
        Ok(())
    }
}
