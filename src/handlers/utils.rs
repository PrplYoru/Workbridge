use crate::models::{
    auth::TokenInfo,
    utils::{UserDetailsA, UserDetailsD},
};
use actix_web::web::Data;
use actix_web::{web, HttpResponse, Responder, HttpRequest};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use mysql::prelude::Queryable;
use mysql::serde_json::{json, Value};
use mysql::{params, serde_json, Pool};

pub async fn get_categories(pool: Data<Pool>) -> impl Responder {
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .json(json!({"message": "Errore di connessione al database"}))
        }
    };

    let result: Vec<(i32, String, String)> = match conn.query("SELECT * FROM categorie") {
        Ok(result) => result,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .json(json!({"message": "Errore nell'esecuzione della query"}))
        }
    };

    HttpResponse::Ok().json(result)
}

pub async fn submit_user_details(
    req: HttpRequest,
    details: web::Json<Value>,
    pool: Data<Pool>,
) -> impl Responder {
    let token = req.headers().get("Authorization").unwrap().to_str().unwrap();
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .json(json!({"message": "Errore di connessione al database"}))
        }
    };

    let secret = b"@vrs%|fumAjH_N|r}d/W(@/KD!/0F*&),d0$26_R-*gKb=PJF(d,j'wcT&we^[]";
    println!("Token: {}", token);
    let token_data = decode::<TokenInfo>(
        &token,
        &DecodingKey::from_secret(secret),
        &Validation::new(Algorithm::HS256),
    );

    match token_data {
        Ok(data) => {
            let user_type = &data.claims.user_type;
            match user_type.as_str() {
                "D" => {
                    let details: UserDetailsD =
                        serde_json::from_value(details.into_inner()).unwrap();
                    let result = conn.exec_drop(
                        r"INSERT INTO diplomati (id_user, nome, specializzazione, indirizzo_studio, voto_maturita, certificazioni_acquisite, esperienze_lavorative) VALUES (:id_user, :nome, :specializzazione, :indirizzo_studio, :voto_maturita, :certificazioni_acquisite, :esperienze_lavorative)",
                        params! {
                            "id_user" => data.claims.user_id,
                            "nome" => &details.nome, //
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
                                Ok(_) => HttpResponse::Ok()
                                    .json(json!({"message": "Dettagli inseriti con successo"})),
                                Err(_) => HttpResponse::InternalServerError()
                                    .json(json!({"message": "Errore nell'esecuzione della query"})),
                            }
                        }
                        Err(_) => HttpResponse::InternalServerError()
                            .json(json!({"message": "Errore nell'esecuzione della query"})),
                    }
                }
                "A" => {
                    let details: UserDetailsA =
                        serde_json::from_value(details.into_inner()).unwrap();
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
                }
                _ => HttpResponse::BadRequest().json(json!({"message": "Tipo utente non valido"})),
            }
        }
        Err(e) => HttpResponse::InternalServerError()
            .json(json!({"message": format!("Errore nella decodifica del token: {}", e)})),
    }
}

pub async fn get_user_details(req: HttpRequest, pool: Data<Pool>) -> impl Responder {
    let token = req.headers().get("Authorization").unwrap().to_str().unwrap();
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .json(json!({"message": "Errore di connessione al database"}))
        }
    };

    let secret = b"@vrs%|fumAjH_N|r}d/W(@/KD!/0F*&),d0$26_R-*gKb=PJF(d,j'wcT&we^[]";
    let token_data = decode::<TokenInfo>(
        &token,
        &DecodingKey::from_secret(secret),
        &Validation::new(Algorithm::HS256),
    );

    match token_data {
        Ok(data) => {
            let user_type = &data.claims.user_type;
            match user_type.as_str() {
                "D" => {
                    let result: Vec<(i32, i32, String, String, String, String, String, String)> = match conn
                        .exec(
                            r"SELECT * FROM diplomati WHERE id_user = :id_user",
                            params! {
                                "id_user" => data.claims.user_id
                            },
                        ) {
                        Ok(result) => result,
                        Err(_) => {
                            return HttpResponse::InternalServerError()
                                .json(json!({"message": "Errore nell'esecuzione della query"}))
                        }
                    };
                    let result_json: serde_json::Value = result
                        .into_iter()
                        .map(
                            |(
                                 id,
                                 user_id,
                                 nome,
                                 specializzazione,
                                 indirizzo_studio,
                                 voto_maturita,
                                 certificazioni_acquisite,
                                 esperienze_lavorative,
                             )| {
                                json!({
                                    "id": id,
                                    "user_id": user_id,
                                    "nome": nome,
                                    "specializzazione": specializzazione,
                                    "indirizzo_studio": indirizzo_studio,
                                    "voto_maturita": voto_maturita,
                                    "certificazioni_acquisite": certificazioni_acquisite,
                                    "esperienze_lavorative": esperienze_lavorative
                                })
                            },
                        )
                        .next()
                        .unwrap_or(json!({}));
                    HttpResponse::Ok().json(result_json)
                }
                "A" => {
let result: Vec<(i32, i32, String, String, String, String, String, String, String, String,)> = match conn
                        .exec(
                            r"SELECT * FROM aziende WHERE id_user = :id_user",
                            params! {
                                "id_user" => data.claims.user_id
                            },
                        ) {
                        Ok(result) => result,
                        Err(_) => {
                            return HttpResponse::InternalServerError()
                                .json(json!({"message": "Errore nell'esecuzione della query"}))
                        }
                    };
                    let result_json: serde_json::Value = result
                        .into_iter()
                        .map(
                            |(
                                 id,
                                 user_id,
                                 denominazione_azienda,
                                 numero_rea,
                                 codice_fiscale,
                                 forma_giuridica,
                                 descrizione_attivita,
                                 categoria,
                                 indirizzo,
                                 contatti,
                             )| {
                                json!({
                                    "id": id,
                                    "user_id": user_id,
                                    "denominazione_azienda": denominazione_azienda,
                                    "numero_rea": numero_rea,
                                    "codice_fiscale": codice_fiscale,
                                    "forma_giuridica": forma_giuridica,
                                    "descrizione_attivita": descrizione_attivita,
                                    "categoria": categoria,
                                    "indirizzo": indirizzo,
                                    "contatti": contatti
                                })
                            },
                        )
                        .next()
                        .unwrap_or(json!({}));
                    HttpResponse::Ok().json(result_json)
                }
                _ => HttpResponse::BadRequest().json(json!({"message": "Tipo utente non valido"})),
            }
        }
        Err(e) => HttpResponse::InternalServerError()
            .json(json!({"message": format!("Errore nella decodifica del token: {}", e)})),
    }
}

pub async fn get_all_users(req: HttpRequest, pool: Data<Pool>) -> impl Responder {
    let token = req.headers().get("Authorization").unwrap().to_str().unwrap();
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .json(json!({"message": "Errore di connessione al database"}))
        }
    };

    let secret = b"@vrs%|fumAjH_N|r}d/W(@/KD!/0F*&),d0$26_R-*gKb=PJF(d,j'wcT&we^[]";
    let token_data = decode::<TokenInfo>(
        &token,
        &DecodingKey::from_secret(secret),
        &Validation::new(Algorithm::HS256),
    );

    match token_data {
        Ok(data) => {
            let user_type = &data.claims.user_type;
            match user_type.as_str() {
                "A" => {
                    let result: Vec<(i32, i32, String, String, String, String, String, String)> = match conn
                        .exec(
                            r"SELECT * FROM diplomati",
                            (),
                        ) {
                        Ok(result) => result,
                        Err(_) => {
                            return HttpResponse::InternalServerError()
                                .json(json!({"message": "Errore nell'esecuzione della query"}))
                        }
                    };
                    let result_json: serde_json::Value = result
                        .into_iter()
                        .map(
                            |(
                                 id,
                                 user_id,
                                 nome,
                                 specializzazione,
                                 indirizzo_studio,
                                 voto_maturita,
                                 certificazioni_acquisite,
                                 esperienze_lavorative,
                             )| {
                                json!({
                                    "id": id,
                                    "user_id": user_id,
                                    "nome": nome,
                                    "specializzazione": specializzazione,
                                    "indirizzo_studio": indirizzo_studio,
                                    "voto_maturita": voto_maturita,
                                    "certificazioni_acquisite": certificazioni_acquisite,
                                    "esperienze_lavorative": esperienze_lavorative
                                })
                            },
                        )
                        .collect();
                    HttpResponse::Ok().json(result_json)
                }
                "D" => {
                    let result: Vec<(i32, i32, String, String, String, String, String, String, String, String, )> = match conn
                        .exec(
                            r"SELECT * FROM aziende",
                            (),
                        ) {
                        Ok(result) => result,
                        Err(_) => {
                            return HttpResponse::InternalServerError()
                                .json(json!({"message": "Errore nell'esecuzione della query"}))
                        }
                    };
                    let result_json: serde_json::Value = result
                        .into_iter()
                        .map(
                            |(
                                 id,
                                 user_id,
                                 denominazione_azienda,
                                 numero_rea,
                                 codice_fiscale,
                                 forma_giuridica,
                                 descrizione_attivita,
                                 categoria,
                                 indirizzo,
                                 contatti,
                             )| {
                                json!({
                                    "id": id,
                                    "user_id": user_id,
                                    "denominazione_azienda": denominazione_azienda,
                                    "numero_rea": numero_rea,
                                    "codice_fiscale": codice_fiscale,
                                    "forma_giuridica": forma_giuridica,
                                    "descrizione_attivita": descrizione_attivita,
                                    "categoria": categoria,
                                    "indirizzo": indirizzo,
                                    "contatti": contatti
                                })
                            },
                        )
                        .collect();
                    HttpResponse::Ok().json(result_json)
                }
                _ => HttpResponse::BadRequest().json(json!({"message": "Tipo utente non valido"})),
            }
        }
        Err(e) => HttpResponse::InternalServerError()
            .json(json!({"message": format!("Errore nella decodifica del token: {}", e)})),
    }
}