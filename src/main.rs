use actix_cors::Cors;
use actix_files::Files;
use actix_web::{web, App, HttpServer, web::Data};
use mysql::{Pool, prelude::Queryable};
use handlers::auth::{login_user, register_user};
use handlers::utils::{submit_user_details, get_categories, get_user_details, get_all_users, get_provinces};

mod handlers {
    pub mod auth;
    pub mod utils;
}

mod models {
    pub mod auth;
    pub mod utils;
}

async fn setup_database(url: &str) -> std::io::Result<Pool> {
    let pool = Pool::new(url).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    let mut conn = pool.get_conn().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    let tables = vec![
        "CREATE TABLE IF NOT EXISTS users (id INT AUTO_INCREMENT PRIMARY KEY, email VARCHAR(60), password VARCHAR(60), tipo_utente VARCHAR(1), verified BOOLEAN DEFAULT FALSE)",
        "CREATE TABLE IF NOT EXISTS diplomati (id INT AUTO_INCREMENT PRIMARY KEY, id_user INT, FOREIGN KEY (id_user) REFERENCES users(id), nome VARCHAR(50), specializzazione VARCHAR(50), indirizzo_studio VARCHAR(50), voto_maturita VARCHAR(3), certificazioni_acquisite VARCHAR(50), esperienze_lavorative VARCHAR(50))",
        "CREATE TABLE IF NOT EXISTS province (id CHAR(2) PRIMARY KEY, name VARCHAR(60))",
        "CREATE TABLE IF NOT EXISTS aziende (id INT AUTO_INCREMENT PRIMARY KEY, id_user INT, FOREIGN KEY(id_user) REFERENCES users(id), denominazione_azienda VARCHAR(50), numero_REA VARCHAR(15), codice_fiscale VARCHAR(16), forma_giuridica VARCHAR(50), descrizione_attivita VARCHAR(50), categoria VARCHAR(50), provincia VARCHAR(2), indirizzo VARCHAR(50), contatti VARCHAR(50))",
        "CREATE TABLE IF NOT EXISTS categorie (id INT AUTO_INCREMENT PRIMARY KEY, name VARCHAR(50), description VARCHAR(255))",
        "CREATE TABLE IF NOT EXISTS cvs (id INT AUTO_INCREMENT PRIMARY KEY, file_path VARCHAR(255) NOT NULL, diplomato_user_id INT, azienda_user_id INT, FOREIGN KEY (diplomato_user_id) REFERENCES users(id), FOREIGN KEY (azienda_user_id) REFERENCES users(id))",
    ];
    for table in tables {
        conn.query_drop(table).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    }
    Ok(pool)
}

fn setup_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/register", web::post().to(register_user))
            .route("/login", web::post().to(login_user))
            .route("/categories", web::get().to(get_categories))
            .route("/provinces", web::get().to(get_provinces))
            .route("/details", web::post().to(submit_user_details))
            .route("/details", web::get().to(get_user_details))
            .route("/users", web::get().to(get_all_users))
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let url = "mysql://root@localhost:3306/diplomati";
    let pool = setup_database(url).await?;
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(Data::new(pool.clone()))
            .configure(setup_routes)
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