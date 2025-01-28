mod api;

use dotenv::dotenv;
use std::env;
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenv().ok();
    let postgres_url = env::var("POSTGRES_URL").expect("POSTGRES_URL must be set");
    let mysql_url = env::var("MYSQL_URL").expect("MYSQL_URL must be set");
    let mongo_url = env::var("MONGO_URL").expect("MONGO_URL must be set");

    // Initialize logger
    tracing_subscriber::fmt::init();
    info!("Starting the federated query engine...");

    // Start the API server
    api::run_api(postgres_url, mysql_url, mongo_url).await;
}
