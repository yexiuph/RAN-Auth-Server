use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct UserInfo {
    pub UserNum: i32,
    pub UserID: String,
    pub UserPass: String,
    pub UserAvailable: i32,
    pub UserBlock: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserInfoResponse {
    pub UserNum: i32,
    pub UserID: String,
    pub UserPass: String,
    pub UserAvailable: i32,
    pub UserBlock: i32,
}
