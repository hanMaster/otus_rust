use std::net::SocketAddr;
use std::sync::Arc;
use server::pool::ClientPool;
use server::bc::Broadcast;
use server::con::Connector;

fn main()-> std::io::Result<()> {
    let pool = Arc::new(ClientPool::default());
    Broadcast::run(pool.clone());
    let mut connector = Connector::new();
    let listener = connector.bind("127.0.0.1:7000")?;
    for stream in listener.incoming() {
        connector.handle_client(stream?, pool.clone())?;
    }
    Ok(())
}
