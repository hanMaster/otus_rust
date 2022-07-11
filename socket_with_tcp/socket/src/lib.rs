use std::fmt::{Display, Formatter};
use std::io;
use std::net::{TcpListener, ToSocketAddrs};

pub enum SocketState {
    OFF,
    ON,
}

impl Display for SocketState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            SocketState::ON => write!(f, "ON"),
            SocketState::OFF => write!(f, "OFF"),
        }
    }
}

pub struct Socket {
    description: String,
    state: SocketState,
    current_power_consumption: f64,
}

impl Default for Socket {
    fn default() -> Self {
        Self::new()
    }
}

impl Socket {
    pub fn new() -> Self {
        let description = String::from("new socket");
        Self {
            description,
            state: SocketState::OFF,
            current_power_consumption: 0.0,
        }
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    pub fn set_description(&mut self, new_description: &str) {
        self.description = String::from(new_description);
    }

    pub fn get_state(&self) -> String {
        match self.state {
            SocketState::ON => String::from("Switched ON"),
            SocketState::OFF => String::from("Switched OFF"),
        }
    }

    pub fn toggle_switch(&mut self) {
        if let SocketState::OFF = self.state {
            self.state = SocketState::ON;
        } else {
            self.state = SocketState::OFF;
        }
    }

    pub fn get_current_power_consumption(&self) -> f64 {
        self.current_power_consumption
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_description() {
        let mut socket = Socket::new();
        socket.set_description("my socket");
        assert_eq!("my socket", socket.get_description());
    }

    #[test]
    fn test_toggle() {
        let mut socket = Socket::new();
        assert_eq!("Switched OFF", socket.get_state());
        socket.toggle_switch();
        assert_eq!("Switched ON", socket.get_state());
    }
}
