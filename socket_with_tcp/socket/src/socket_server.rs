use std::io;
use std::io::{Read, Write};
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
        match decoded.as_slice() {
            b"cmd0" => {
                println!("Socket status requested");
                let state = socket.get_state();
                let pwr = socket.get_current_power_consumption();
                println!("State: {}, pwr: {:.2} Watt", if state == 1 {"ON"} else {"OFF"}, pwr);
                stream.write(b"qwertyui")?;
            },
            b"cmd1" => {
                println!("Socket switch on invoked");
                socket.switch_on();
            },
            b"cmd2" => {
                println!("Socket switch off invoked");
                socket.switch_off();
            },
            _ => println!("received: bad command")
        }
        Ok(())
    }
}
