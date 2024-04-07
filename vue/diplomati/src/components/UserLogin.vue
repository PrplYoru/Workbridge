<template>
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
    <v-btn type="submit" color="primary" class="mr-4 form-button">Login</v-btn>
    <v-btn type="button" color="secondary" class="form-button" @click="register">Registrati</v-btn>
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
          // The user was registered successfully
          // Redirect the user to the login page
        } else {
          // There was an error
          // Show an error message
        }
      } catch (error) {
        // There was an error sending the request
        // Show an error message
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
