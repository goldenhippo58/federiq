pub mod api;
pub mod connectors;
pub mod metrics;
pub mod query_executor;
pub mod query_optimizer;

#[cfg(test)]
mod tests;


#[cfg(test)]
mod tests {
    use super::query_executor::QueryExecutor;
    use super::query_optimizer::QueryOptimizer;

    #[tokio::test]
    async fn test_federated_queries() {
        let postgres_url = "postgres://user:password@localhost:15432/testdb";
        let mysql_url = "mysql://user:password@localhost:3307/testdb";
        let mongo_url = "mongodb://localhost:27019";
        let redis_url = "redis://127.0.0.1/";

        let executor = QueryExecutor::new(postgres_url, mysql_url, mongo_url, redis_url)
            .await
            .unwrap();
        let unified_data = executor.fetch_and_merge_data().await.unwrap();

        assert!(!unified_data.is_empty());
    }

    #[tokio::test]
    async fn test_query_optimizer() {
        let optimizer = QueryOptimizer::new();
        let sources = vec!["postgres", "mysql", "mongo"];
        let plans = optimizer.optimize_query(sources).await;

        assert_eq!(plans.len(), 3);
        assert!(plans.iter().all(|plan| plan.cost > 0.0));
    }
}
