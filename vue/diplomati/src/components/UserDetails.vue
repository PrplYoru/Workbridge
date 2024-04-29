<template>
  <v-card class="form-card">
    <v-card-title>
      <h2 class="form-title" v-if="user_type === 'D'">Dettagli Diplomato</h2>
      <h2 class="form-title" v-else-if="user_type === 'A'">Dettagli Azienda</h2>
    </v-card-title>
    <v-form @submit.prevent="submitDetails" class="form" ref="form">
      <template v-if="user_type === 'D'">
        <v-text-field label="Nome" v-model="details.nome" required></v-text-field>
        <v-text-field label="Specializzazione" v-model="details.specializzazione" required></v-text-field>
        <v-text-field label="Indirizzo di Studio" v-model="details.indirizzo_studio" required></v-text-field>
        <v-text-field label="Voto di Maturità" v-model="details.voto_maturita" required></v-text-field>
        <v-text-field label="Certificazioni Acquisite" v-model="details.certificazioni_acquisite" required></v-text-field>
        <v-text-field label="Esperienze Lavorative" v-model="details.esperienze_lavorative" required></v-text-field>
      </template>
      <template v-else-if="user_type === 'A'">
        <v-text-field label="Denominazione Azienda" v-model="details.denominazione_azienda" required></v-text-field>
        <v-text-field label="Numero REA" v-model="details.numero_rea" required></v-text-field>
        <v-text-field label="Codice Fiscale" v-model="details.codice_fiscale" required></v-text-field>
        <v-text-field label="Forma Giuridica" v-model="details.forma_giuridica" required></v-text-field>
        <v-text-field label="Descrizione Attività" v-model="details.descrizione_attivita" required></v-text-field>
        <v-select label="Categoria" v-model="details.categoria" :items="categories" required></v-select>
        <v-select label="Provincia" v-model="details.provincia" :items="provinces" required></v-select>
        <v-text-field label="Indirizzo" v-model="details.indirizzo" required></v-text-field>
        <v-text-field label="Contatti" v-model="details.contatti" required></v-text-field>
      </template>
      <v-btn type="submit" color="primary" class="mr-4 form-button" @click="validate">Submit</v-btn>
    </v-form>
  </v-card>
</template>

<script>
import { jwtDecode } from 'jwt-decode';
import axios from 'axios';
export default {
  data() {
    const token = localStorage.getItem('token');
    const decodedToken = jwtDecode(token);
    return {
      provinces: [],
      categories: [],
      user_type: decodedToken.user_type,
      details: {
        nome: '',
        specializzazione: '',
        indirizzo_studio: '',
        voto_maturita: '',
        certificazioni_acquisite: '',
        esperienze_lavorative: '',
        denominazione_azienda: '',
        numero_rea: '',
        codice_fiscale: '',
        forma_giuridica: '',
        descrizione_attivita: '',
        categoria: '',
        provincia: '',
        indirizzo: '',
        contatti: '',
      },
    };
  },
  async created() {
    try {
      await this.fetchProvinces();
      const response = await axios.get('http://127.0.0.1:8000/api/categories');
      this.categories = response.data.map(category => category[1]);
    } catch (error) {
      console.error(error);
    }
  },
  methods: {
    async fetchProvinces() {
      try {
        const response = await axios.get('http://127.0.0.1:8000/api/provinces');
        this.provinces = response.data.map(province => province[1]);
      } catch (error) {
        console.error(error);
      }
    },
    async submitDetails() {
      const token = localStorage.getItem('token');
      try {
        const response = await axios.post(`/api/details`, this.details, {
          headers: {
            Authorization: `${token}`,
          },
        });
        if (response.status === 200) {
          this.$router.push('/profile');
        } else {
          alert(response.data.message)
        }
      } catch (error) {
        alert(error.response.data.message)
      }
    },
  },
};
</script>

<style scoped>
.form-card {
  max-width: 90%;
  margin: auto 33%;
  padding: 20px;
  box-shadow: 0px 0px 10px rgba(0, 0, 0, 0.1);
  border-radius: 10px;
  background-color: #f5f5f5;
}

.form-title {
  font-size: 24px;
  font-weight: bold;
  color: #333;
  font-family: 'Roboto', sans-serif;
}

.form {
  margin-top: 20px;
}

.form-field {
  margin-bottom: 20px;
  transition: all 0.3s ease;
  border-radius: 5px;
  padding: 10px;
}

.form-field:focus {
  transform: scale(1.05);
  border-color: #007BFF;
}

.form-button {
  font-weight: bold;
  transition: all 0.3s ease;
  border-radius: 5px;
  padding: 10px 20px;
}

.form-button:hover {
  transform: scale(1.05);
  background-color: #007BFF;
  color: #fff;
}
</style>