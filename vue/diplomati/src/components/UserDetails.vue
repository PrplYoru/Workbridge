<template>
  <v-form @submit.prevent="submitDetails" class="form">
    <template v-if="user_type === 'D'">
      <v-card-title>
        <h2>Dettagli Diplomato</h2>
      </v-card-title>
      <v-text-field label="Specializzazione" v-model="details.specializzazione" required></v-text-field>
      <v-text-field label="Indirizzo di Studio" v-model="details.indirizzo_studio" required></v-text-field>
      <v-text-field label="Voto di Maturità" v-model="details.voto_maturita" required></v-text-field>
      <v-text-field label="Certificazioni Acquisite" v-model="details.certificazioni_acquisite" required></v-text-field>
      <v-text-field label="Esperienze Lavorative" v-model="details.esperienze_lavorative" required></v-text-field>
    </template>
    <template v-else-if="user_type === 'A'">
      <v-card-title>
        <h2>Dettagli Azienda</h2>
      </v-card-title>
      <v-text-field label="Denominazione Azienda" v-model="details.denominazione_azienda" required></v-text-field>
      <v-text-field label="Numero REA" v-model="details.numero_REA" required></v-text-field>
      <v-text-field label="Codice Fiscale" v-model="details.codice_fiscale" required></v-text-field>
      <v-text-field label="Forma Giuridica" v-model="details.forma_giuridica" required></v-text-field>
      <v-text-field label="Descrizione Attività" v-model="details.descrizione_attivita" required></v-text-field>
      <v-text-field label="Categoria" v-model="details.categoria" required></v-text-field>
      <v-text-field label="Indirizzo" v-model="details.indirizzo" required></v-text-field>
      <v-text-field label="Contatti" v-model="details.contatti" required></v-text-field>
    </template>
    <v-btn type="submit" color="secondary" class="form-button">Submit</v-btn>
  </v-form>
</template>

<script>
import { jwtDecode } from 'jwt-decode';
import axios from 'axios';
export default {
  data() {
    const token = localStorage.getItem('token');
    const decodedToken = jwtDecode(token);
    return {
      user_type: decodedToken.user_type,
      details: {
        specializzazione: '',
        indirizzo_studio: '',
        voto_maturita: '',
        certificazioni_acquisite: '',
        esperienze_lavorative: '',
        denominazione_azienda: '',
        numero_REA: '',
        codice_fiscale: '',
        forma_giuridica: '',
        descrizione_attivita: '',
        categoria: '',
        indirizzo: '',
        contatti: '',
      },
    };
  },
  methods: {
    async submitDetails() {
      const token = localStorage.getItem('token');
      try {
        const response = await axios.post(`/api/details/${token}`, this.details);
        console.log(response.data);
      } catch (error) {
        console.error(error);
      }
    },
  },
};
</script>

<style scoped>
.form {
  max-width: 500px;
  margin: auto;
}
.form-button {
  transition: all 0.3s ease;
}
.form-button:hover {
  transform: scale(1.05);
}
</style>