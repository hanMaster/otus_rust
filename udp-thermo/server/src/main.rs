use server::bc::Broadcast;
use server::con::Connector;
use server::pool::ClientPool;
use std::sync::Arc;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let pool = Arc::new(ClientPool::default());
    Broadcast::run(pool.clone());
    let mut connector = Connector::default();
    let listener = connector.bind("127.0.0.1:7000").await?;

    loop {
        let (stream, _) = listener.accept().await?;
        if connector.handle_client(stream, pool.clone()).await.is_err() {
            println!("Failed to handle connection");
        };
    }
}
