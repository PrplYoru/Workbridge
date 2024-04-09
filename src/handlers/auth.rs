use actix_web::{web, HttpResponse, Responder};
use actix_web::web::Data;
use mysql::prelude::Queryable;
use mysql::{params, Pool};
use mysql::serde_json::json;
use crate::models::auth::{UserInfo, LoginInfo};

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

    let select_stmt = r"SELECT password, verified FROM users WHERE email = :email";
    let result: Option<(String, bool)> = match conn.exec_first(select_stmt, params! {"email" => &info.email}) {
        Ok(result) => result,
        Err(_) => return HttpResponse::InternalServerError().json(json!({"message": "Errore nell'esecuzione della query"})),
    };

    match result {
        Some((stored_password, verified)) => {
            if bcrypt::verify(&info.password, &stored_password).unwrap() {
                if verified {
                    HttpResponse::Ok().json(json!({"message": "Login effettuato con successo", "verified": true}))
                } else {
                    HttpResponse::Ok().json(json!({"message": "Login effettuato con successo", "verified": false}))
                }
            } else {
                HttpResponse::BadRequest().json(json!({"message": "Password errata"}))
            }
        },
        None => HttpResponse::BadRequest().json(json!({"message": "Utente non trovato"})),
    }
}