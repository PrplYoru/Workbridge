<template>
  <v-card class="form-card">
    <v-card-title>
      <h2 class="form-title">Registrazione</h2>
    </v-card-title>
    <v-form @submit.prevent="register" class="form" ref="form" v-model="validForm">
      <v-text-field
          label="Email"
          type="email"
          v-model="email"
          :rules="emailRules"
          required
          class="form-field"
          placeholder="Enter your email"
      ></v-text-field>
      <v-text-field
          label="Password"
          type="password"
          v-model="password"
          :rules="passwordRules"
          required
          class="form-field"
          placeholder="Enter your password"
      ></v-text-field>
      <v-select
          label="User Type"
          v-model="tipo_utente"
          :items="['Diplomato', 'Azienda']"
          :rules="userTypeRules"
          required
          class="form-field"
          placeholder="Select your user type"
      ></v-select>
      <v-btn type="submit" color="primary" class="mr-4 form-button" :disabled="!validForm">Registrati</v-btn>
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
      validForm: false,
    };
  },
  computed: {
    emailRules() {
      return [
        v => !!v || 'Email is required',
        v => /.+@.+\..+/.test(v) || 'Invalid email',
      ];
    },
    passwordRules() {
      return [
        v => !!v || 'Password is required',
      ];
    },
    userTypeRules() {
      return [
        v => !!v || 'User type is required',
      ];
    },
  },
  methods: {
    async register() {
      try {
        if (this.tipo_utente === 'Diplomato') {
          this.tipo_utente = 'D';
        } else if(this.tipo_utente === 'Azienda') {
          this.tipo_utente = 'A';
        } else {
          this.$vuetify.toast('Invalid user type');
          return;
        }
        const response = await axios.post('http://127.0.0.1:8000/api/register', {
          email: this.email,
          password: this.password,
          tipo_utente: this.tipo_utente,
        });

        if (response.status === 200) {
          this.$router.push('/login');
          this.$vuetify.toast(response.data.message);
        } else {
          alert(response.data.message);
        }
      } catch (error) {
        alert(error.response.data.message);
      }
    },
    login() {
      this.$router.push('/login');
    },
  },
};
</script>

<style scoped>
:root {
  --primary-color: #007BFF;
  --secondary-color: #f5f5f5;
  --font-color: #333;
  --font-family: 'Roboto', sans-serif;
}

.form-card {
  max-width: 90%;
  margin: auto 33%;
  padding: 20px;
  box-shadow: 0px 0px 10px rgba(0, 0, 0, 0.1);
  border-radius: 10px;
  background-color: var(--secondary-color);
}

.form-title {
  font-size: 24px;
  font-weight: bold;
  color: var(--font-color);
  font-family: var(--font-family),serif;
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
  border-color: var(--primary-color);
}

.form-button {
  font-weight: bold;
  transition: all 0.3s ease;
  border-radius: 5px;
  padding: 10px 20px;
}

.form-button:hover {
  transform: scale(1.05);
  background-color: var(--primary-color);
  color: #fff;
}
</style>
