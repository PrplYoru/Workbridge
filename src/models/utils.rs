use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserDetailsD {
    pub specializzazione: String,
    pub indirizzo_studio: String,
    pub voto_maturita: i32,
    pub certificazioni_acquisite: String,
    pub esperienze_lavorative: String,
}

#[derive(Deserialize)]
pub struct UserDetailsA {
    pub denominazione_azienda: String,
    pub numero_REA: i32,
    pub codice_fiscale: String,
    pub forma_giuridica: String,
    pub descrizione_attivita: String,
    pub categoria: String,
    pub indirizzo: String,
    pub contatti: String,
}