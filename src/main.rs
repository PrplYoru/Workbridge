use actix_cors::Cors;
use actix_files::Files;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use mysql::prelude::Queryable;
use mysql::{Pool};
use handlers::utils::{submit_user_details, get_categories, get_user_details, get_all_users};
use handlers::auth::{login_user, register_user};
use crate::handlers::utils::get_provinces;

mod handlers {
    pub mod auth;
    pub mod utils;
}

mod models {
    pub mod auth;
    pub mod utils;
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
        "CREATE TABLE IF NOT EXISTS diplomati (id INT AUTO_INCREMENT PRIMARY KEY, id_user INT, FOREIGN KEY (id_user) REFERENCES users(id), nome VARCHAR(50), specializzazione VARCHAR(50), indirizzo_studio VARCHAR(50), voto_maturita VARCHAR(3), certificazioni_acquisite VARCHAR(50), esperienze_lavorative VARCHAR(50))",
        "CREATE TABLE IF NOT EXISTS aziende (id INT AUTO_INCREMENT PRIMARY KEY, id_user INT, FOREIGN KEY(id_user) REFERENCES users(id), denominazione_azienda VARCHAR(50), numero_REA VARCHAR(15), codice_fiscale VARCHAR(16), forma_giuridica VARCHAR(50), descrizione_attivita VARCHAR(50), categoria VARCHAR(50), provincia CHAR(2), FOREIGN KEY(provincia) REFERENCES province(id), indirizzo VARCHAR(50), contatti VARCHAR(50))",
        "CREATE TABLE IF NOT EXISTS categorie (id INT AUTO_INCREMENT PRIMARY KEY, name VARCHAR(50), description VARCHAR(255))",
        "CREATE TABLE IF NOT EXISTS province (id CHAR(2) PRIMARY KEY, name VARCHAR(60))",
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
            .route("/api/categories", web::get().to(get_categories))
            .route("/api/provinces", web::get().to(get_provinces))
            .route("/api/details", web::post().to(submit_user_details))
            .route("/api/details", web::get().to(get_user_details))
            .route("/api/users", web::get().to(get_all_users))
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
