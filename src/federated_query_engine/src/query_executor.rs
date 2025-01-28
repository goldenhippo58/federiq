use crate::connectors::{MongoConnector, MySqlConnector, PostgresConnector};
use crate::metrics::observe_query_latency;
use crate::query_optimizer::QueryOptimizer;
use deadpool_redis::{Config as RedisConfig, Pool as RedisPool, Runtime};
use futures_util::TryStreamExt;
use mongodb::bson::doc;
use serde::Serialize;
use sqlx::Row;

#[derive(Debug, Serialize, Clone)] // Added Clone derive
pub struct UnifiedUser {
    pub id: i32,
    pub name: String,
    pub email: Option<String>,
    pub department: Option<String>,
}


pub struct QueryExecutor {
    postgres: PostgresConnector,
    mysql: MySqlConnector,
    mongo: MongoConnector,
    #[allow(dead_code)]
    redis_pool: RedisPool,
    optimizer: QueryOptimizer,
}

impl QueryExecutor {
    pub async fn new(
        postgres_url: &str,
        mysql_url: &str,
        mongo_url: &str,
        redis_url: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let postgres = PostgresConnector::new(postgres_url).await?;
        let mysql = MySqlConnector::new(mysql_url).await?;
        let mongo = MongoConnector::new(mongo_url).await?;
        let redis_pool = RedisConfig::from_url(redis_url).create_pool(Some(Runtime::Tokio1))?;
        let optimizer = QueryOptimizer::new();

        Ok(Self {
            postgres,
            mysql,
            mongo,
            redis_pool,
            optimizer,
        })
    }

    pub async fn fetch_and_merge_data(
        &self,
    ) -> Result<Vec<UnifiedUser>, Box<dyn std::error::Error>> {
        let sources = vec!["postgres", "mysql", "mongo"];
        let execution_plan = self.optimizer.optimize_query(sources).await;

        let mut unified_users = Vec::new();
        let mut seen = std::collections::HashSet::new(); // To remove duplicates

        for plan in execution_plan {
            match plan.source.as_str() {
                "postgres" => {
                    let postgres_result = self.postgres.execute_query("SELECT * FROM users").await;
                    if let Ok(rows) = postgres_result {
                        observe_query_latency("postgres", plan.cost);
                        for row in rows {
                            let user = UnifiedUser {
                                id: row.get("id"),
                                name: row.get("name"),
                                email: Some(row.get("email")),
                                department: None,
                            };
                            if seen.insert((user.id, user.name.clone(), user.email.clone())) {
                                unified_users.push(user);
                            }
                        }
                    }
                }
                "mysql" => {
                    let mysql_result = self.mysql.execute_query("SELECT * FROM employees").await;
                    if let Ok(rows) = mysql_result {
                        observe_query_latency("mysql", plan.cost);
                        for row in rows {
                            let user = UnifiedUser {
                                id: row.get("id"),
                                name: row.get("name"),
                                email: None,
                                department: Some(row.get("department")),
                            };
                            if seen.insert((user.id, user.name.clone(), user.email.clone())) {
                                unified_users.push(user);
                            }
                        }
                    }
                }
                "mongo" => {
                    let collection = self.mongo.get_collection("testdb", "users").await;
                    let mut cursor = match collection.find(doc! {}).await {
                        Ok(cursor) => cursor,
                        Err(err) => {
                            eprintln!("MongoDB error: {:?}", err);
                            return Ok(unified_users); // Return the results gathered so far
                        }
                    };
                    while let Some(doc) = cursor.try_next().await.unwrap_or(None) {
                        let id: i32 = doc.get_i32("id").unwrap_or(0);
                        let name: String = doc.get_str("name").unwrap_or("").to_string();
                        let email: Option<String> =
                            doc.get_str("email").ok().map(|s| s.to_string());
                        let user = UnifiedUser {
                            id,
                            name,
                            email,
                            department: None,
                        };
                        if seen.insert((user.id, user.name.clone(), user.email.clone())) {
                            unified_users.push(user);
                        }
                    }
                }

                _ => {}
            }
        }

        Ok(unified_users)
    }
}
