use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use serde_json::json;
use sqlx::mssql::{MssqlPoolOptions, MssqlPool};

pub struct AppState {
    db: MssqlPool,
}

impl AppState {
    async fn check_connection(&self) -> Result<(), sqlx::Error> {
        let _ = self.db.acquire().await?;
        Ok(())
    }
}

#[get("/api/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    HttpResponse::Ok().json(json!({"status": "success", "message": "OK"}))
}

#[get("/api/databasechecker")]
async fn database_checker_handler(config: web::Data<AppState>) -> impl Responder {
    match config.check_connection().await {
        Ok(_) => HttpResponse::Ok().json(json!({"status": "success", "message": "Database connection is healthy."})),
        Err(err) => {
            println!("{}", err);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": format!("Database connection failed: {}", err)}))
        },
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }    
    env_logger::init();

    let database_url = "sqlserver://yxgames:yxgamesdev123@127.0.0.1:1433/RanUser";
    let pool = match MssqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .service(health_checker_handler)
            .service(database_checker_handler)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 2000))?
    .run()
    .await
}
