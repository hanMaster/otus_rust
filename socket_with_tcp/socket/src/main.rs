use crate::socket_server::SocketServer;
use socket::Socket;

mod socket_server;

// cargo run --package client
fn main() -> std::io::Result<()> {
    let mut socket = Socket::new();
    socket.set_description("Socket with TCP server");

    let listener = SocketServer::bind("127.0.0.1:7000")?;
    for stream in listener.incoming() {
        SocketServer::handle_client(stream?)?;
    }
    Ok(())
}
