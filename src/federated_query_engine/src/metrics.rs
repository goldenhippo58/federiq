use lazy_static::lazy_static;
use prometheus::{Encoder, HistogramVec, IntCounter, TextEncoder};

lazy_static! {
    static ref POSTGRES_QUERIES: IntCounter =
        IntCounter::new("postgres_queries", "Number of PostgreSQL queries").unwrap();
    static ref MYSQL_QUERIES: IntCounter =
        IntCounter::new("mysql_queries", "Number of MySQL queries").unwrap();
    static ref MONGO_QUERIES: IntCounter =
        IntCounter::new("mongo_queries", "Number of MongoDB queries").unwrap();
    static ref QUERY_LATENCY: HistogramVec = HistogramVec::new(
        prometheus::opts!("query_latency", "Latency of database queries").into(),
        &["database"]
    )
    .unwrap();
}

pub fn observe_query_latency(database: &str, duration: f64) {
    QUERY_LATENCY
        .with_label_values(&[database])
        .observe(duration);
}

pub async fn metrics_handler() -> String {
    let encoder = TextEncoder::new();
    let mut buffer = Vec::new();
    let metrics = prometheus::gather();
    encoder.encode(&metrics, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}
