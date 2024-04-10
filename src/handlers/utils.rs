use actix_web::{web, HttpResponse, Responder};
use actix_web::web::Data;
use mysql::prelude::Queryable;
use mysql::{params, Pool, serde_json};
use mysql::serde_json::{json, Value};
use crate::models::{utils::{UserDetailsD, UserDetailsA}, auth::TokenInfo};
use jsonwebtoken::{decode, Validation, DecodingKey, Algorithm};
use jsonwebtoken::errors::Result as JWTResult;

pub async fn submit_user_details(details: web::Json<Value>, token: web::Path<String>, pool: Data<Pool>) -> impl Responder {
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().json(json!({"message": "Errore di connessione al database"})),
    };

    let secret = b"@vrs%|fumAjH_N|r}d/W(@/KD!/0F*&),d0$26_R-*gKb=PJF(d,j'wcT&we^[]";
    println!("Token: {}", token);
    let token_data = decode::<TokenInfo>(&token, &DecodingKey::from_secret(secret), &Validation::new(Algorithm::HS256));

    match token_data {
        Ok(data) => {
            let user_type = &data.claims.user_type;
            match user_type.as_str() {
                "D" => {
                    let details: UserDetailsD = serde_json::from_value(details.into_inner()).unwrap();
                    let result = conn.exec_drop(
                        r"INSERT INTO diplomati (id_user, specializzazione, indirizzo_studio, voto_maturita, certificazioni_acquisite, esperienze_lavorative) VALUES (:id_user , :specializzazione, :indirizzo_studio, :voto_maturita, :certificazioni_acquisite, :esperienze_lavorative)",
                        params! {
                            "id_user" => data.claims.user_id,
                            "specializzazione" => &details.specializzazione,
                            "indirizzo_studio" => &details.indirizzo_studio,
                            "voto_maturita" => &details.voto_maturita,
                            "certificazioni_acquisite" => &details.certificazioni_acquisite,
                            "esperienze_lavorative" => &details.esperienze_lavorative
                        },
                    );
                    match result {
                        Ok(_) => {
                            let verify_result = conn.exec_drop(
                                r"UPDATE users SET verified = true WHERE id = :id",
                                params! {
                                    "id" => data.claims.user_id
                                },
                            );
                            match verify_result {
                                Ok(_) => HttpResponse::Ok().json(json!({"message": "Dettagli inseriti con successo"})),
                                Err(_) => HttpResponse::InternalServerError().json(json!({"message": "Errore nell'esecuzione della query"})),
                            }
                        },
                        Err(_) => HttpResponse::InternalServerError().json(json!({"message": "Errore nell'esecuzione della query"})),
                    }
                },
                "A" => {
                    let details: UserDetailsA = serde_json::from_value(details.into_inner()).unwrap();
                    let result = conn.exec_drop(
                        r"INSERT INTO aziende (id_user, denominazione_azienda, numero_rea, codice_fiscale, forma_giuridica, descrizione_attivita, categoria, indirizzo, contatti) VALUES (:id_user, :denominazione_azienda, :numero_rea, :codice_fiscale, :forma_giuridica, :descrizione_attivita, :categoria, :indirizzo, :contatti)",
                        params! {
                            "id_user" => data.claims.user_id,
                            "denominazione_azienda" => &details.denominazione_azienda,
                            "numero_rea" => &details.numero_rea,
                            "codice_fiscale" => &details.codice_fiscale,
                            "forma_giuridica" => &details.forma_giuridica,
                            "descrizione_attivita" => &details.descrizione_attivita,
                            "categoria" => &details.categoria,
                            "indirizzo" => &details.indirizzo,
                            "contatti" => &details.contatti
                        },
                    );
                    match result {
                        Ok(_) => {
                            let verify_result = conn.exec_drop(
                                r"UPDATE users SET verified = true WHERE id = :id",
                                params! {
                                    "id" => data.claims.user_id
                                },
                            );
                            match verify_result {
                                Ok(_) => HttpResponse::Ok().json(json!({"message": "Dettagli inseriti con successo"})),
                                Err(_) => HttpResponse::InternalServerError().json(json!({"message": "Errore nell'esecuzione della query"})),
                            }
                        },
                        Err(e) => HttpResponse::InternalServerError().json(json!({"message": format!("Errore nell'esecuzione della query: {}", e.to_string())})),
                    }
                },
                _ => HttpResponse::BadRequest().json(json!({"message": "Tipo utente non valido"})),
            }
        },
        Err(e) => HttpResponse::InternalServerError().json(json!({"message": format!("Errore nella decodifica del token: {}", e)})),
    }
}