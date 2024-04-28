<template>
  <v-card class="form-card">
    <v-card-title>
      <h2 class="form-title">Registrazione</h2>
    </v-card-title>
    <v-form @submit.prevent="register" class="form" ref="form">
      <v-text-field
          label="Email"
          type="email"
          v-model="email"
          :rules="[rules.required, rules.email]"
          required
          class="form-field"
          placeholder="Enter your email"
      ></v-text-field>
      <v-text-field
          label="Password"
          type="password"
          v-model="password"
          :rules="[rules.required]"
          required
          class="form-field"
          placeholder="Enter your password"
      ></v-text-field>
      <v-select
          label="User Type"
          v-model="tipo_utente"
          :items="['Diplomato', 'Azienda']"
          :rules="[rules.required]"
          required
          class="form-field"
          placeholder="Select your user type"
      ></v-select>
      <v-btn type="submit" color="primary" class="mr-4 form-button" @click="validate">Registrati</v-btn>
      <v-btn type="button" color="secondary" class="form-button" @click="login">Login</v-btn>
    </v-form>
  </v-card>
</template>

<script>
import axios from 'axios';

export default {
  data() {
    return {
      email: '',
      password: '',
      tipo_utente: '',
      rules: {
        required: value => !!value || 'Required.',
        email: value => /.+@.+\..+/.test(value) || 'Invalid email.',
      },
    };
  },
  methods: {
    async register() {
      if (!this.email || !this.password || !this.tipo_utente) {
        alert('Inserisci tutti i campi');
        return;
      }
      if (!/.+@.+\..+/.test(this.email)) {
        alert('Email non valida');
        return;
      }

      try {
        if (this.tipo_utente === 'Diplomato') {
          this.tipo_utente = 'D';
        } else if(this.tipo_utente === 'Azienda') {
          this.tipo_utente = 'A';
        } else {
          alert('Invalid user type');
          return;
        }
        const response = await axios.post('http://127.0.0.1:8000/api/register', {
          email: this.email,
          password: this.password,
          tipo_utente: this.tipo_utente,
        });

        if (response.status === 200) {
          this.$router.push('/login');
          alert(response.data.message);
        } else {
          alert(response.data.message)
        }
      } catch (error) {
        alert(error.response.data.message);
      }
    },
    login() {
      this.$router.push('/login');
    },
    validate() {
      this.$refs.form.validate();
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
