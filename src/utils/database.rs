use std::sync::Arc; // Import Arc
use sqlx::mssql::{MssqlPoolOptions, MssqlPool};

pub struct DatabaseState {
    pub db_pool: Arc<MssqlPool>,
}

impl DatabaseState {
    pub async fn check_connection(&self) -> Result<(), sqlx::Error> {
        let _ = self.db_pool.acquire().await?;
        Ok(())
    }
}

pub async fn connect_database(db_url: String) -> Arc<MssqlPool> {
    match MssqlPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await
    {
        Ok(db_pool) => {
            println!("✅ Connection to the database is successful!");
            Arc::new(db_pool)
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    }
}