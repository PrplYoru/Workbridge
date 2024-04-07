use actix_cors::Cors;
use actix_files::Files;
use actix_web::web::Data;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use mysql::prelude::Queryable;
use mysql::{params, Pool};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserInfo {
    email: String,
    password: String,
    tipo_utente: String,
}

async fn register_user(info: web::Json<UserInfo>, pool: Data<Pool>) -> impl Responder {
    let mut conn = pool.get_conn();
    if conn.is_err() {
        eprintln!("Error getting connection: {:?}", conn.err());
        return HttpResponse::InternalServerError().body("Error getting connection");
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
        return HttpResponse::BadRequest().body("Email already exists");
    }

    let hashed_password = bcrypt::hash(&info.password, bcrypt::DEFAULT_COST);
    if hashed_password.is_err() {
        eprintln!("Error hashing password: {:?}", hashed_password.err());
        return HttpResponse::InternalServerError().body("Error hashing password");
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
        Ok(_) => HttpResponse::Ok().body("User registered successfully"),
        Err(e) => {
            eprintln!("Error executing query: {:?}", e);
            HttpResponse::InternalServerError().body("Error executing query")
        }
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
        "CREATE TABLE IF NOT EXISTS users (id INT AUTO_INCREMENT PRIMARY KEY, email VARCHAR(60), password VARCHAR(50), tipo_utente VARCHAR(1))",
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
            .route("/register", web::post().to(register_user))
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
