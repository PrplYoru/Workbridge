<template>
  <v-card class="form-card">
    <v-card-title>
      <h2 class="form-title">{{ formTitle }}</h2>
    </v-card-title>
    <v-form @submit.prevent="submitDetails" class="form" ref="form">
      <template v-for="field in formFields">
        <v-text-field v-if="field.type === 'text'" :label="field.label" v-model="details[field.model]" :key="field.model" required></v-text-field>
        <v-select v-else-if="field.type === 'select'" :label="field.label" v-model="details[field.model]" :items="field.items" :key="field.model" required></v-select>
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
  computed: {
    formTitle() {
      return this.user_type === 'D' ? 'Dettagli Diplomato' : 'Dettagli Azienda';
    },
    formFields() {
      return this.user_type === 'D' ? [
        { type: 'text', label: 'Nome', model: 'nome' },
        { type: 'text', label: 'Specializzazione', model: 'specializzazione'},
        { type: 'text', label: 'Indirizzo di Studio', model: 'indirizzo_studio' },
        { type: 'text', label: 'Voto Maturità', model: 'voto_maturita' },
        { type: 'text', label: 'Certificazioni Acquisite', model: 'certificazioni_acquisite' },
        { type: 'text', label: 'Esperienze Lavorative', model: 'esperienze_lavorative' },
      ] : [
        { type: 'text', label: 'Denominazione Azienda', model: 'denominazione_azienda' },
        { type: 'text', label: 'Numero REA', model: 'numero_rea' },
        { type: 'text', label: 'Codice Fiscale', model: 'codice_fiscale' },
        { type: 'text', label: 'Forma Giuridica', model: 'forma_giuridica' },
        { type: 'text', label: 'Descrizione Attività', model: 'descrizione_attivita' },
        { type: 'select', label: 'Categoria', model: 'categoria', items: this.categories },
        { type: 'select', label: 'Provincia', model: 'provincia', items: this.provinces },
        { type: 'text', label: 'Indirizzo', model: 'indirizzo' },
        { type: 'text', label: 'Contatti', model: 'contatti' },
      ];
    },
  },
  async created() {
    try {
      await this.fetchData('http://127.0.0.1:8000/api/provinces', 'provinces');
      await this.fetchData('http://127.0.0.1:8000/api/categories', 'categories');
    } catch (error) {
      this.handleError(error);
    }
  },
  methods: {
    async fetchData(url, dataKey) {
      const response = await axios.get(url);
      this[dataKey] = response.data.map(item => item[1]);
    },
    handleError(error) {
      console.error(error);
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
        this.handleError(error);
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