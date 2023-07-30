use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct AuthResult {
    pub message: String,
    pub status: bool,
}

#[derive(Debug, Deserialize, Serialize,)]
pub struct AuthInfo {
    pub user_id: String,
    pub user_pass: String,
}