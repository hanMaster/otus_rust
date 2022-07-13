use std::io;
use std::io::{ErrorKind, Read, Write};
use std::net::TcpStream;
use socket::cmd::Commands;
use socket::crypt::Crypt;

pub struct SocketClient{}

impl SocketClient {
    pub fn get_status() -> io::Result<String> {
        let mut stream = TcpStream::connect("127.0.0.1:7000")?;
        stream.write(&Commands::get_status_cmd())?;
        let mut buf: [u8;12] = [0;12];
        stream.read_exact(&mut buf)?;
        let msg = buf.as_slice().decrypt();
        if &msg[0..3] == b"skt" {
            let state = msg[3];
            let mut pwr_buf:[u8;8] = [0;8];
            pwr_buf[0] = msg[4];
            pwr_buf[1] = msg[5];
            pwr_buf[2] = msg[6];
            pwr_buf[3] = msg[7];
            pwr_buf[4] = msg[8];
            pwr_buf[5] = msg[9];
            pwr_buf[6] = msg[10];
            pwr_buf[7] = msg[11];
            let pwr = f64::from_be_bytes(pwr_buf);
            Ok(format!("Socket state: {}, power: {:.2}", if state == 0 {"OFF"} else {"ON"}, pwr))
        } else {
            Err(io::Error::new(ErrorKind::InvalidData, "Corrupted message"))
        }
    }

    pub fn switch_on() -> io::Result<()> {
        let mut stream = TcpStream::connect("127.0.0.1:7000")?;
        stream.write(&Commands::switch_on_cmd())?;
        Ok(())
    }

    pub fn switch_off() -> io::Result<()> {
        let mut stream = TcpStream::connect("127.0.0.1:7000")?;
        stream.write(&Commands::switch_off_cmd())?;
        Ok(())
    }
}