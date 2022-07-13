use concat_arrays::concat_arrays;
use socket::crypt::Crypt;
use socket::Socket;
use std::io;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};

pub struct SocketServer {}

impl SocketServer {
    pub fn bind<Addrs>(addrs: Addrs) -> io::Result<TcpListener>
    where
        Addrs: ToSocketAddrs,
    {
        let tcp = TcpListener::bind(addrs)?;
        Ok(tcp)
    }
    /// command: byte array [u8;4]:
    /// cmd: prefix
    /// 0: get status
    /// 1: switch on
    /// 2: switch off
    ///
    /// response: byte array [u8;12]
    /// skt: prefix
    /// ['s', 'k', 't']
    /// 1 byte - ON or OFF
    /// 8 byte - f64 (power consumption)

    pub fn handle_client(mut stream: TcpStream, socket: &mut Socket) -> io::Result<()> {
        let mut buf = [0; 4];
        stream.read_exact(&mut buf)?;
        let decoded = buf.as_slice().decrypt();
        match decoded.as_slice() {
            b"cmd0" => {
                println!("Socket status requested");
                let state = socket.get_state();
                let pwr = socket.get_current_power_consumption();
                println!(
                    "State: {}, pwr: {:.2} Watt",
                    if state == 1 { "ON" } else { "OFF" },
                    pwr
                );
                let prefix = b"skt";
                let pwr_bytes = pwr.to_be_bytes();
                let response: [u8; 12] = concat_arrays!(*prefix, [state], pwr_bytes);
                stream.write_all(&response.as_slice().encrypt())?;
            }
            b"cmd1" => {
                println!("Socket switch on invoked");
                socket.switch_on();
            }
            b"cmd2" => {
                println!("Socket switch off invoked");
                socket.switch_off();
            }
            _ => println!("received: bad command"),
        }
        Ok(())
    }
}
