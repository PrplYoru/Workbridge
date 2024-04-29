use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserDetailsD {
    pub nome: String,
    pub specializzazione: String,
    pub indirizzo_studio: String,
    pub voto_maturita: String,
    pub certificazioni_acquisite: String,
    pub esperienze_lavorative: String,
}

#[derive(Deserialize)]
pub struct UserDetailsA {
    pub denominazione_azienda: String,
    pub numero_rea: String,
    pub codice_fiscale: String,
    pub forma_giuridica: String,
    pub descrizione_attivita: String,
    pub categoria: String,
    pub provincia: String,
    pub indirizzo: String,
    pub contatti: String,
}