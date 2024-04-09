use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct UserInfo {
    pub email: String,
    pub password: String,
    pub tipo_utente: String,
}

#[derive(Deserialize)]
pub struct LoginInfo {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct TokenInfo {
    pub user_id: i32,
    pub email: String,
    pub user_type: String,
}
