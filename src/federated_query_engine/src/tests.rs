#[cfg(test)]
mod tests {
    use crate::query_executor::QueryExecutor;
    use crate::query_optimizer::QueryOptimizer;

    #[tokio::test]
    async fn test_federated_queries() {
        let executor = QueryExecutor::new(
            "postgres://user:password@localhost:15432/testdb",
            "mysql://user:password@localhost:3307/testdb",
            "mongodb://localhost:27019",
            "redis://127.0.0.1/",
        )
        .await
        .unwrap();

        let data = executor.fetch_and_merge_data().await.unwrap();
        assert!(!data.is_empty());
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
