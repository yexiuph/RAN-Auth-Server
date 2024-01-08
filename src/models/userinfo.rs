use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct AuthResult {
    pub message: String,
    pub status: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_num: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthInfo {
    pub user_id: String,
    pub user_pass: String,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct RegisterResult {
    pub message: String,
    pub status: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterInfo {
    pub user_id: String,
    pub user_email: String,
    pub user_pass: String,
    pub user_pass2: String,
}
