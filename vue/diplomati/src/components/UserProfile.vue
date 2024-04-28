<template>
  <div class="user-profile">
    <div class="profile-card" v-if="user_type === 'D'">
      <div class="card-header">
        <h2>Hello, {{ details.nome }}</h2>
      </div>
      <div class="card-body">
        <div class="profile-detail" v-for="(value, key) in details" :key="key">
          <h3>{{ key }}: </h3>
          <p>{{ value }}</p>
        </div>
      </div>
    </div>
    <div class="profile-card" v-else-if="user_type === 'A'">
      <div class="card-header">
        <h2>Company Details: {{ details.denominazione_azienda }}</h2>
      </div>
      <div class="card-body">
        <div class="profile-detail" v-for="(value, key) in details" :key="key">
          <h3>{{ key }}: </h3>
          <p>{{ value }}</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import {jwtDecode} from 'jwt-decode';
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
      const response = await axios.get(`/api/details`, {
        headers: {
          Authorization: `${token}`,
        },
      });
      this.details = response.data;
    } catch (error) {
      console.error(error);
    }
  },
};
</script>

<style scoped>
@keyframes fadeIn {
  0% {opacity: 0;}
  100% {opacity: 1;}
}

.user-profile {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100vh;
  background: linear-gradient(to right, #f5f5f5, #e0e0e0);
}

.profile-card {
  width: 80%;
  margin: 20px;
  background-color: #fff;
  border-radius: 10px;
  box-shadow: 0px 0px 10px rgba(0, 0, 0, 0.1);
  overflow: hidden;
  animation: fadeIn 1s;
}

.card-header {
  background-color: #007BFF;
  color: #fff;
  padding: 20px;
  font-size: 1.5em;
}

.card-body {
  padding: 20px;
}

.profile-detail {
  display: flex;
  justify-content: space-between;
  border-bottom: 1px solid #f0f0f0;
  padding: 15px 0;
  transition: background-color 0.3s ease, transform 0.3s ease;
}

.profile-detail:hover {
  background-color: #f0f0f0;
  transform: scale(1.02);
}

.profile-detail:last-child {
  border-bottom: none;
}

.profile-detail h3 {
  font-weight: bold;
  color: #333;
  font-size: 1.2em;
}

.profile-detail p {
  color: #666;
  font-size: 1em;
}
</style>
