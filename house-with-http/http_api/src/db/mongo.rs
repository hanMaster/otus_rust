use dotenv::dotenv;
use mongodb::{Client, Database};
use std::env;
use std::sync::Arc;

#[derive(Debug)]
pub struct Mongo {
    db: Arc<Database>,
}

impl Mongo {
    pub async fn new() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v,
            Err(_) => "Error loading env variable".to_string(),
        };
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("smart-house");
        Self { db: Arc::new(db) }
    }

    pub fn get_db(&self) -> Arc<Database> {
        Arc::clone(&self.db)
    }
}
