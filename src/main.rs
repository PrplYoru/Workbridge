use actix_cors::Cors;
use actix_files::Files;
use actix_web::web::Data;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use mysql::prelude::Queryable;
use mysql::{params, Pool};
use mysql::serde_json::json;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserInfo {
    email: String,
    password: String,
    tipo_utente: String,
}

#[derive(Deserialize)]
pub struct LoginInfo {
    email: String,
    password: String,
}

async fn register_user(info: web::Json<UserInfo>, pool: Data<Pool>) -> impl Responder {
    let mut conn = pool.get_conn();
    if conn.is_err() {
        return HttpResponse::InternalServerError().body("Error di connessione al database");
    }
    let mut conn = conn.unwrap();

    let select_stmt = r"SELECT email FROM users WHERE email = :email";
    let existing_email: Option<String> = conn
        .exec_first(
            select_stmt,
            params! {
                "email" => &info.email,
            },
        )
        .unwrap();

    if existing_email.is_some() {
        return HttpResponse::BadRequest().body("Email già esistente");
    }

    let hashed_password = bcrypt::hash(&info.password, bcrypt::DEFAULT_COST);
    if hashed_password.is_err() {
        return HttpResponse::InternalServerError().body("Errore nell'hashing della password");
    }
    let hashed_password = hashed_password.unwrap();

    let result = conn.exec_drop(
        r"INSERT INTO users (email, password, tipo_utente) VALUES (:email, :password, :tipo_utente)",
        params! {
            "email" => &info.email,
            "password" => hashed_password,
            "tipo_utente" => &info.tipo_utente
        },
    );

    match result {
        Ok(_) => HttpResponse::Ok().body("Utente registrato con successo"),
        Err(e) => {
            HttpResponse::InternalServerError().body("Errore nell'esecuzione della query")
        }
    }
}

async fn login_user(info: web::Json<LoginInfo>, pool: Data<Pool>) -> impl Responder {
    let mut conn = pool.get_conn();
    if conn.is_err() {
        return HttpResponse::InternalServerError().json(json!({"message": "Errore di connessione al database"}));
    }
    let mut conn = conn.unwrap();

    let select_stmt = r"SELECT password, verified FROM users WHERE email = :email";
    let result: Option<(String, bool)> = conn
        .exec_first(
            select_stmt,
            params! {
                "email" => &info.email,
            },
        )
        .unwrap();

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let url = "mysql://root@localhost:3306/diplomati";
    let pool = match Pool::new(url) {
        Ok(pool) => pool,
        Err(e) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            ))
        }
    };

    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            ))
        }
    };

    let tables = vec![
        "CREATE TABLE IF NOT EXISTS users (id INT AUTO_INCREMENT PRIMARY KEY, email VARCHAR(60), password VARCHAR(60), tipo_utente VARCHAR(1), verified BOOLEAN DEFAULT FALSE)",
        "CREATE TABLE IF NOT EXISTS diplomati (id INT AUTO_INCREMENT PRIMARY KEY, id_user INT, FOREIGN KEY (id_user) REFERENCES users(id), specializzazione VARCHAR(50), indirizzo_studio VARCHAR(50), voto_maturita INT, certificazioni_acquisite VARCHAR(50), esperienze_lavorative VARCHAR(50))",
        "CREATE TABLE IF NOT EXISTS aziende (id INT AUTO_INCREMENT PRIMARY KEY, denominazione_azienda VARCHAR(50), numero_REA INT, codice_fiscale VARCHAR(16), forma_giuridica VARCHAR(50), descrizione_attivita VARCHAR(50), categoria VARCHAR(50), indirizzo VARCHAR(50), contatti VARCHAR(50))",
    ];

    for table in tables {
        match conn.query_drop(table) {
            Ok(_) => (),
            Err(e) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    e.to_string(),
                ))
            }
        };
    }

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .app_data(Data::new(pool.clone()))
            .route("/api/register", web::post().to(register_user))
            .route("/api/login", web::post().to(login_user))
            .service(Files::new("/", "vue/diplomati/dist").index_file("index.html"))
            .default_service(
                web::route().to(|| async {
                    Ok::<_, std::io::Error>(actix_files::NamedFile::open("vue/diplomati/dist/index.html")?)
                }),
            )
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
