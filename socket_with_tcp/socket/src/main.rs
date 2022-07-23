use crate::socket_server::SocketServer;
use socket::Socket;

mod socket_server;

// cargo run --package socket
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut socket = Socket::new();
    socket.set_description("Socket with TCP server");

    let listener = SocketServer::bind("127.0.0.1:7000").await?;

    loop {
        let (stream, _) = listener.accept().await?;
        if SocketServer::handle_client(stream, &mut socket)
            .await
            .is_err()
        {
            println!("Failed to handle connection");
        };
    }
}
