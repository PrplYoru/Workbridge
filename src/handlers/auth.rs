use actix_web::{web, HttpResponse, Responder};
use actix_web::web::Data;
use mysql::prelude::Queryable;
use mysql::{params, Pool};
use mysql::serde_json::json;
use crate::models::auth::{UserInfo, LoginInfo, TokenInfo};
use jsonwebtoken::{encode, Header, EncodingKey, Algorithm};
use jsonwebtoken::errors::Result as JWTResult;

fn generate_token(user_id: i32, email: &str, user_type: &str) -> JWTResult<String> {
    let claims = TokenInfo {
        user_id,
        email: email.to_string(),
        user_type: user_type.to_string(),
        exp: 10000000000,
    };

    let secret = b"@vrs%|fumAjH_N|r}d/W(@/KD!/0F*&),d0$26_R-*gKb=PJF(d,j'wcT&we^[]";
    encode(&Header::new(Algorithm::HS256), &claims, &EncodingKey::from_secret(secret))
}

pub async fn register_user(info: web::Json<UserInfo>, pool: Data<Pool>) -> impl Responder {
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().json(json!({"message": "Errore di connessione al database"})),
    };

    let select_stmt = r"SELECT email FROM users WHERE email = :email";
    let existing_email: Option<String> = match conn.exec_first(select_stmt, params! {"email" => &info.email}) {
        Ok(email) => email,
        Err(_) => return HttpResponse::InternalServerError().json(json!({"message": "Errore nell'esecuzione della query"})),
    };

    if existing_email.is_some() {
        return HttpResponse::BadRequest().json(json!({"message": "Email già registrata"}))
    }

    let hashed_password = match bcrypt::hash(&info.password, bcrypt::DEFAULT_COST) {
        Ok(password) => password,
        Err(_) => return HttpResponse::InternalServerError().json(json!({"message": "Errore nell'hashing della password"})),
    };

    let result = conn.exec_drop(
        r"INSERT INTO users (email, password, tipo_utente) VALUES (:email, :password, :tipo_utente)",
        params! {
            "email" => &info.email,
            "password" => hashed_password,
            "tipo_utente" => &info.tipo_utente
        },
    );

    match result {
        Ok(_) => HttpResponse::Ok().json(json!({"message": "Utente registrato con successo"})),
        Err(_) => HttpResponse::InternalServerError().json(json!({"message": "Errore nell'esecuzione della query"})),
    }
}

pub async fn login_user(info: web::Json<LoginInfo>, pool: Data<Pool>) -> impl Responder {
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().json(json!({"message": "Errore di connessione al database"})),
    };

    let select_stmt = r"SELECT id, password, tipo_utente, verified FROM users WHERE email = :email";
    let result: Option<(i32, String, String, bool)> = match conn.exec_first(select_stmt, params! {"email" => &info.email}) {
        Ok(result) => result,
        Err(_) => return HttpResponse::InternalServerError().json(json!({"message": "Errore nell'esecuzione della query"})),
    };

    match result {
        Some((user_id, stored_password, user_type, verified)) => {
            if bcrypt::verify(&info.password, &stored_password).unwrap() {
                if verified {
                    let token = match generate_token(user_id, &info.email, &user_type) {
                        Ok(token) => token,
                        Err(_) => return HttpResponse::InternalServerError().json(json!({"message": "Errore nella generazione del token"})),
                    };
                    HttpResponse::Ok().json(json!({"message": "Login effettuato con successo", "verified": true, "token": token}))
                } else {
                    let token = match generate_token(user_id, &info.email, &user_type) {
                        Ok(token) => token,
                        Err(_) => return HttpResponse::InternalServerError().json(json!({"message": "Errore nella generazione del token"})),
                    };
                    HttpResponse::Ok().json(json!({"message": "Login effettuato con successo", "verified": false, "token": token}))
                }
            } else {
                HttpResponse::BadRequest().json(json!({"message": "Password errata"}))
            }
        },
        None => HttpResponse::BadRequest().json(json!({"message": "Utente non trovato"})),
    }
}