use serde::Deserialize;

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
