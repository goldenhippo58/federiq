use sqlx::{mysql::MySqlPool, postgres::PgPool};
use mongodb::{Client, options::ClientOptions, Collection, bson::Document};

pub struct PostgresConnector {
    pool: PgPool,
}

impl PostgresConnector {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPool::connect(database_url).await?;
        Ok(Self { pool })
    }

    pub async fn execute_query(&self, query: &str) -> Result<Vec<sqlx::postgres::PgRow>, sqlx::Error> {
        let rows = sqlx::query(query).fetch_all(&self.pool).await?;
        Ok(rows)
    }
}

pub struct MySqlConnector {
    pool: MySqlPool,
}

impl MySqlConnector {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = MySqlPool::connect(database_url).await?;
        Ok(Self { pool })
    }

    pub async fn execute_query(&self, query: &str) -> Result<Vec<sqlx::mysql::MySqlRow>, sqlx::Error> {
        let rows = sqlx::query(query).fetch_all(&self.pool).await?;
        Ok(rows)
    }
}

pub struct MongoConnector {
    client: Client,
}

impl MongoConnector {
    pub async fn new(database_url: &str) -> Result<Self, mongodb::error::Error> {
        let options = ClientOptions::parse(database_url).await?;
        let client = Client::with_options(options)?;
        Ok(Self { client })
    }

    pub async fn get_collection(&self, db_name: &str, collection_name: &str) -> Collection<Document> {
        self.client.database(db_name).collection(collection_name)
    }
}
