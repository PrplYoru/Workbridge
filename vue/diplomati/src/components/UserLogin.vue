<template>
  <v-card-title>
    <h2>Login</h2>
  </v-card-title>
  <v-form @submit.prevent="login" class="form">
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
    <v-btn type="button" color="primary" class="mr-4 form-button" @click="register">Registrati</v-btn>
    <v-btn type="submit" color="secondary" class="form-button">Login</v-btn>
  </v-form>
</template>

<script>
import axios from 'axios';

export default {
  data() {
    return {
      email: '',
      password: '',
    };
  },
  methods: {
    async login() {
      try {
        const response = await axios.post('http://127.0.0.1:8000/api/login', {
          email: this.email,
          password: this.password,
        });

        if (response.status === 200) {
          alert(response.data)
        } else {
          alert(response.data)
        }
      } catch (error) {
        alert(error)
      }
    },
    register() {
      this.$router.push('/register');
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
