<template>
  <div class="user-profile">
    <v-card v-if="user_type === 'D'">
      <v-card-title>
        <h2>Ciao, {{ details.nome }}</h2>
      </v-card-title>
      <v-list>
        <v-list-item>
          <v-list-item-title>Specializzazione: {{ details.specializzazione }}</v-list-item-title>
        </v-list-item>
        <v-list-item>
          <v-list-item-title>Indirizzo di Studio: {{ details.indirizzo_studio }}</v-list-item-title>
        </v-list-item>
        <v-list-item>
          <v-list-item-title>Voto di Maturità: {{ details.voto_maturita }}</v-list-item-title>
        </v-list-item>
        <v-list-item>
          <v-list-item-title>Certificazioni Acquisite: {{ details.certificazioni_acquisite }}</v-list-item-title>
        </v-list-item>
        <v-list-item>
          <v-list-item-title>Esperienze Lavorative: {{ details.esperienze_lavorative }}</v-list-item-title>
        </v-list-item>
      </v-list>
    </v-card>
    <v-card v-else-if="user_type === 'A'">
      <v-card-title>
        <h2>Dettagli dell'azienda {{ details.denominazione_azienda }}</h2>
      </v-card-title>
      <v-list>
        <v-list-item>
          <v-list-item-title>Denominazione Azienda: {{ details.denominazione_azienda }}</v-list-item-title>
        </v-list-item>
        <v-list-item>
          <v-list-item-title>Numero REA: {{ details.numero_rea }}</v-list-item-title>
        </v-list-item>
        <v-list-item>
          <v-list-item-title>Codice Fiscale: {{ details.codice_fiscale }}</v-list-item-title>
        </v-list-item>
        <v-list-item>
          <v-list-item-title>Forma Giuridica: {{ details.forma_giuridica }}</v-list-item-title>
        </v-list-item>
        <v-list-item>
          <v-list-item-title>Descrizione Attività: {{ details.descrizione_attivita }}</v-list-item-title>
        </v-list-item>
        <v-list-item>
          <v-list-item-title>Categoria: {{ details.categoria }}</v-list-item-title>
        </v-list-item>
        <v-list-item>
          <v-list-item-title>Indirizzo: {{ details.indirizzo }}</v-list-item-title>
        </v-list-item>
        <v-list-item>
          <v-list-item-title>Contatti: {{ details.contatti }}</v-list-item-title>
        </v-list-item>
      </v-list>
    </v-card>
  </div>
</template>

<script>
import { jwtDecode } from 'jwt-decode';
import axios from 'axios';

export default {
  data() {
    return {
      user_type: '',
      details: {},
    };
  },
  async created() {
    const token = localStorage.getItem('token');
    const decodedToken = jwtDecode(token);
    this.user_type = decodedToken.user_type;

    try {
      const response = await axios.get(`/api/details/${token}`);
      this.details = response.data;
    } catch (error) {
      console.error(error);
    }
  },
};
</script>

<style scoped>
.user-profile {
  max-width: 500px;
  margin: auto;
  font-family: Arial, sans-serif;
  color: #333;
  animation: fadeIn 1s ease-in;
}

@keyframes fadeIn {
  0% { opacity: 0; }
  100% { opacity: 1; }
}

.v-card {
  box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2);
  transition: 0.3s;
  border-radius: 5px;
  background-color: #f0f0f0;
  margin-bottom: 20px;
  animation: slideIn 1s ease-in;
}

@keyframes slideIn {
  0% { transform: translateY(-50px); opacity: 0; }
  100% { transform: translateY(0); opacity: 1; }
}

.v-card:hover {
  box-shadow: 0 8px 16px 0 rgba(0,0,0,0.2);
  transform: scale(1.02);
}

.v-card-title {
  padding: 16px;
  border-bottom: 1px solid #f0f0f0;
  background-color: #007BFF;
  color: #fff;
}

.v-card-title h2 {
  margin: 0;
  font-size: 1.5em;
}

.v-list {
  padding: 16px;
}

.v-list-item {
  padding: 10px 0;
  border-bottom: 1px solid #f0f0f0;
  transition: background-color 0.3s ease;
}

.v-list-item:hover {
  background-color: #f0f0f0;
}

.v-list-item:last-child {
  border-bottom: none;
}
</style>