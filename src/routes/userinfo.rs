use crate::{
    models::userinfo::{AuthInfo, AuthResult, RegisterInfo, RegisterResult},
    ConfigState, DatabaseState,
};
use actix_web::{error::InternalError, post, web, HttpResponse, Responder};
use serde_json::{json, Value};
use sqlx::{query, query_file, query_scalar, Column, Row, TypeInfo};

// #[post("/gmlogin")]
// pub async fn gm_login(
//     configData: web::Data<ConfigState>,
//     database: web::Data<DatabaseState>,
//     auth_info: web::Json<AuthInfo>,
// ) -> Result<HttpResponse, actix_web::Error> {

//     let sql_query = 
// }

#[post("/login")]
pub async fn verify_user_and_pass(
    configData: web::Data<ConfigState>,
    database: web::Data<DatabaseState>,
    auth_info: web::Json<AuthInfo>,
) -> Result<HttpResponse, actix_web::Error> {
    let sql_query = format!(
        "EXEC {}.dbo.GSAuth @UserID = N'{}', @UserPass = N'{}'",
        configData.config.get_ran_user_db(),
        &auth_info.user_id,
        &auth_info.user_pass
    );

    let pool = &database.db_pool;
    let mut conn = pool
        .acquire()
        .await
        .expect("Failed to acquire a connection from the pool.");

    let result: Result<(bool, i32), sqlx::Error> =
        sqlx::query_as(&sql_query).fetch_one(&mut conn).await;

    println!("{:?}", result);   
    match result {
        Ok((auth_status, user_num)) => {
            let auth_response = match auth_status {
                true => AuthResult {
                    message: "Authentication successful.".to_string(),
                    status: true,
                    user_num: Some(user_num),
                },
                false => AuthResult {
                    message: "Incorrect password or user not found.".to_string(),
                    status: false,
                    user_num: None,
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

#[post("/register")]
pub async fn register_user_info(
    configData: web::Data<ConfigState>,
    database: web::Data<DatabaseState>,
    register_info: web::Json<RegisterInfo>,
) -> Result<HttpResponse, actix_web::Error> {
    let sql_query = format!(
        "EXEC {}.dbo.GSRegister @userId = N'{}', @userMail = N'{}', @userPass1 = N'{}', @userPass2 = N'{}'",
        configData.config.get_ran_user_db(),
        &register_info.user_id,
        &register_info.user_email,
        &register_info.user_pass,
        &register_info.user_pass2
    );

    let pool = &database.db_pool;
    let mut conn = pool
        .acquire()
        .await
        .expect("Failed to acquire a connection from the pool.");

    let result: Result<i32, sqlx::Error> =
        sqlx::query_scalar(&sql_query).fetch_one(&mut conn).await;

    match result {
        Ok(n_return) => {
            if n_return == 0 {
                // User already exists
                Ok(HttpResponse::Ok().json(RegisterResult {
                    message: "User already exists.".to_string(),
                    status: false,
                }))
            } else if n_return == 1 {
                // User registered successfully
                Ok(HttpResponse::Ok().json(RegisterResult {
                    message: "User registered successfully.".to_string(),
                    status: true,
                }))
            } else {
                // An error occurred during registration
                Ok(HttpResponse::InternalServerError().finish())
            }
        }
        Err(e) => {
            eprintln!("Error during user registration: {:?}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(register_user_info);
    cfg.service(verify_user_and_pass);
}
