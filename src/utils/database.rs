use chrono::Duration;
use sqlx::mssql::{MssqlPool, MssqlPoolOptions};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration as StdDuration;

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
    let max_retries = 5;
    let retry_interval = Duration::seconds(5);

    for retry_attempt in 1..=max_retries {
        match MssqlPoolOptions::new()
            .max_connections(10)
            .connect(&db_url)
            .await
        {
            Ok(db_pool) => {
                println!("🟢 Connection to the database is successful!");
                return Arc::new(db_pool);
            }
            Err(err) => {
                println!("🔴 Failed to connect to the database: {:?}", err);
                if retry_attempt < max_retries {
                    println!("Retrying in {} seconds...", retry_interval.num_seconds());
                    sleep(StdDuration::from_secs(retry_interval.num_seconds() as u64));
                }
            }
        }
    }

    println!(
        "❌ Max retries reached. Unable to connect to the database after {} retries.",
        max_retries
    );
    std::process::exit(1);
}
