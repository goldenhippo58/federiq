use crate::query_executor::QueryExecutor;
use axum::{extract::Query, routing::get, Json, Router};
use serde::Deserialize;
use serde_json::json;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::signal;

#[derive(Deserialize)]
pub struct QueryParams {
    pub page: Option<usize>,
    pub size: Option<usize>,
    pub filter: Option<String>, // Example: "name=Alice" or "department=HR"
    pub source: Option<String>, // Example: "postgres,mysql"
}

#[axum::debug_handler]
async fn fetch_federated_query(Query(params): Query<QueryParams>) -> Json<serde_json::Value> {
    let postgres_url = "postgres://user:password@postgres_db:5432/testdb";
    let mysql_url = "mysql://user:password@mysql_db:3306/testdb";
    let mongo_url = "mongodb://mongodb_db:27017";
    let redis_url = "redis://redis_db:6379";

    // Create a QueryExecutor
    let executor = QueryExecutor::new(postgres_url, mysql_url, mongo_url, redis_url)
        .await
        .unwrap();

    // Fetch and merge data
    let mut unified_data = executor.fetch_and_merge_data().await.unwrap();

    // Apply filtering
    if let Some(filter) = &params.filter {
        let parts: Vec<&str> = filter.split('=').collect();
        if parts.len() == 2 {
            let key = parts[0];
            let value = parts[1];
            unified_data.retain(|user| {
                if key == "name" {
                    user.name == value
                } else if key == "department" {
                    user.department.as_deref() == Some(value)
                } else {
                    false
                }
            });
        }
    }

    // Filter by source (optional)
    if let Some(source) = &params.source {
        let sources: Vec<&str> = source.split(',').collect();
        unified_data.retain(|user| {
            sources.contains(&"postgres") && user.email.is_some() // PostgreSQL users
                || sources.contains(&"mysql") && user.department.is_some() // MySQL users
                || sources.contains(&"mongo") // MongoDB users
        });
    }

    // Apply pagination
    let page = params.page.unwrap_or(1);
    let size = params.size.unwrap_or(10);
    let start = (page - 1) * size;
    let end = start + size;
    let paginated_data = unified_data[start..std::cmp::min(end, unified_data.len())].to_vec();

    // Return the data as JSON
    Json(json!(paginated_data))
}

/// Metrics API for monitoring the server and queries
#[axum::debug_handler]
async fn metrics() -> &'static str {
    "API Metrics: Query counts, latencies, error rates, etc."
}

pub async fn run_api_server() {
    let app = Router::new()
        .route("/health", get(|| async { "API Server is Running" }))
        .route("/federated_query", get(fetch_federated_query))
        .route("/metrics", get(metrics)); // Add metrics route

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Starting API server at http://{}", addr);

    // Start the server
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Server error");
}

async fn shutdown_signal() {
    let ctrl_c = signal::ctrl_c();

    #[cfg(unix)]
    let mut terminate_signal = signal::unix::signal(signal::unix::SignalKind::terminate())
        .expect("Failed to install signal handler");

    #[cfg(not(unix))]
    let terminate_signal = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate_signal.recv() => {},
    }

    println!("Shutting down gracefully...");
}
