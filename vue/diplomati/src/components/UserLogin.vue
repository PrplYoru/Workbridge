<template>
  <v-card class="form-card">
    <v-card-title>
      <h2 class="form-title">Login</h2>
    </v-card-title>
    <v-form @submit.prevent="login" class="form" ref="form">
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
      <v-btn type="submit" color="primary" class="mr-4 form-button" @click="validate">Login</v-btn>
      <v-btn type="button" color="secondary" class="form-button" @click="register">Registrati</v-btn>
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
      rules: {
        required: value => !!value || 'Required.',
        email: value => /.+@.+\..+/.test(value) || 'Invalid email.',
      },
    };
  },
  methods: {
    async login() {
      if (!this.email || !this.password) {
        alert('Inserisci tutti i campi');
        return;
      }
      if (!/.+@.+\..+/.test(this.email)) {
        alert('Email non valida');
        return;
      }

      try {
        const response = await axios.post('http://127.0.0.1:8000/api/login', {
          email: this.email,
          password: this.password,
        });

        if (response.status === 200) {
          localStorage.setItem('token', response.data.token);
          if (response.data.verified) {
            this.$router.push('/profile');
          } else {
            this.$router.push('/details');
          }
        } else {
          alert(response.data.message)
        }
      } catch (error) {
        alert(error.response.data.message)
      }
    },
    register() {
      this.$router.push('/register');
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