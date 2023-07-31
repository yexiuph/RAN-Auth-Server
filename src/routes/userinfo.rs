use actix_web::{web, post, HttpResponse , Responder, error::InternalError};
use crate::{
    models::userinfo::{
        AuthResult, AuthInfo
    },
    DatabaseState
};
use serde_json::{json, Value};
use sqlx::{query, query_file, query_scalar, Row, Column,TypeInfo};

#[post("/login")]
pub async fn verify_user_and_pass(
    database: web::Data<DatabaseState>,
    auth_info: web::Json<AuthInfo>,
) -> Result<HttpResponse, actix_web::Error> {
    // Extract user_id and user_pass from the request payload
    let user_id = &auth_info.user_id;
    let user_pass = &auth_info.user_pass;

    let sql_query = format!("EXEC VerifyUserAndPassword @UserID = N'{}', @UserPass = N'{}'", user_id, user_pass);
    // Get a database connection from the pool
    let pool = &database.db_pool;
    let mut conn = pool.acquire().await.expect("Failed to acquire a connection from the pool.");

    // Execute the query and get the result as a boolean
    let result: Result<bool, sqlx::Error> = query_scalar(&sql_query).fetch_one(&mut conn).await;

    match result {
        Ok(auth_status) => {
            let auth_response = match auth_status {
                true => AuthResult {
                    message: "Authentication successful.".to_string(),
                    status: true,
                },
                false => AuthResult {
                    message: "Incorrect password or user not found.".to_string(),
                    status: false,
                },
            };
            Ok(HttpResponse::Ok().json(auth_response))
        }
        Err(e) => {
            eprintln!("Error during authentication: {:?}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(verify_user_and_pass);
}