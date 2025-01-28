use axum::{routing::get, Router};
use std::net::SocketAddr;
use tracing::info;

use crate::metrics::metrics_handler;

async fn fetch_data() -> &'static str {
    "Fetched federated data"
}

pub async fn run_api(postgres_url: String, mysql_url: String, mongo_url: String) {
    let app = Router::new()
        .route("/data", get(fetch_data))
        .route("/metrics", get(metrics_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("API server running at http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
