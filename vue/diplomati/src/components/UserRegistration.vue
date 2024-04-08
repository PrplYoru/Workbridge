<template>
  <v-card-title>
    <h2>Registrazione</h2>
  </v-card-title>
  <v-form @submit.prevent="register" class="form">
    <v-text-field
        label="Email"
        type="email"
        v-model="email"
        required
        class="form-field"
    ></v-text-field>
    <v-text-field
        label="Password"
        type="password"
        v-model="password"
        required
        class="form-field"
    ></v-text-field>
    <v-select
        label="User Type"
        v-model="tipo_utente"
        :items="['Diplomato', 'Azienda']"
        required
        class="form-field"
    ></v-select>
    <v-btn type="submit" color="primary" class="mr-4 form-button">Registrati</v-btn>
    <v-btn type="button" color="secondary" class="form-button" @click="login">Login</v-btn>
  </v-form>
</template>

<script>
import axios from 'axios';

export default {
  data() {
    return {
      email: '',
      password: '',
      tipo_utente: '',
    };
  },
  methods: {
    async register() {
      try {
        const response = await axios.post('http://127.0.0.1:8000/api/register', {
          email: this.email,
          password: this.password,
          tipo_utente: this.tipo_utente,
        });

        if (response.status === 200) {
          this.$router.push('/login');
          alert(response.data);
        } else {
          alert(response.data)
        }
      } catch (error) {
        alert(error);
      }
    },
    login() {
      this.$router.push('/login');
    },
  },
};
</script>
<style scoped>
.form {
  max-width: 500px;
  margin: auto;
}

.form-field {
  transition: all 0.3s ease;
}

.form-field:focus {
  transform: scale(1.05);
}

.form-button {
  transition: all 0.3s ease;
}

.form-button:hover {
  transform: scale(1.05);
}
</style>
