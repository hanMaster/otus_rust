use socket::cmd::Commands;
use socket::crypt::Crypt;
use std::io::{ErrorKind, Read, Write, Result, Error};
use std::net::TcpStream;
use crate::SockError::{GetStatusFailed, NoError, SwitchOffFailed, SwitchOnFailed};

pub struct SocketClient {}

impl SocketClient {
    pub fn get_status() -> Result<(u8, f64)> {
        let mut stream = TcpStream::connect("127.0.0.1:7000")?;
        stream.write_all(&Commands::get_status_cmd())?;
        let mut buf: [u8; 12] = [0; 12];
        stream.read_exact(&mut buf)?;
        let msg = buf.as_slice().decrypt();
        if &msg[0..3] == b"skt" {
            let state = msg[3];
            let pwr_buf: [u8; 8] = msg[4..].try_into().unwrap();
            let pwr = f64::from_be_bytes(pwr_buf);
            Ok((state, pwr))
        } else {
            Err(Error::new(ErrorKind::InvalidData, "Corrupted message"))
        }
    }

    pub fn switch_on() -> Result<()> {
        let mut stream = TcpStream::connect("127.0.0.1:7000")?;
        stream.write_all(&Commands::switch_on_cmd())?;
        Ok(())
    }

    pub fn switch_off() -> Result<()> {
        let mut stream = TcpStream::connect("127.0.0.1:7000")?;
        stream.write_all(&Commands::switch_off_cmd())?;
        Ok(())
    }
}

#[repr(i32)]
pub enum SockError {
    NoError = 0,
    SwitchOnFailed,
    SwitchOffFailed,
    GetStatusFailed
}

#[derive(Default)]
#[repr(C)]
pub struct SocketState {
    state: u32,
    power: f64,
    error: i32
}

#[no_mangle]
pub extern "C" fn sync() -> SocketState {
    if let Ok((state, power)) = SocketClient::get_status() {
        return SocketState{
            state: state as u32,
            power,
            error: NoError as i32
        };
    }
    let mut result = SocketState::default();
    result.error = GetStatusFailed as i32;
    result
}

#[no_mangle]
pub extern "C"  fn switch_on() -> SockError {
    if SocketClient::switch_on().is_ok() {
        return NoError;
    }
    SwitchOnFailed
}

#[no_mangle]
pub extern "C"  fn switch_off() -> SockError {
    if SocketClient::switch_off().is_ok(){
        return NoError;
    }
    SwitchOffFailed
}
