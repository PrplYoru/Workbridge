<template>
  <v-container>
    <v-text-field class="search-bar" v-model="search" label="Search"></v-text-field>
    <v-row v-if="user_type === 'D'">
      <v-col cols="4" v-for="(user, index) in filteredUsers" :key="user.id" :class="{ 'mb-4': (index + 1) % 3!== 0 }">
        <v-card @click="userClicked(user)">
          <v-card-title>
            <h2>{{ user.denominazione_azienda }}</h2>
          </v-card-title>
          <v-list>
            <v-list-item>
              <v-list-item-title>Categoria: {{ user.categoria }}</v-list-item-title>
            </v-list-item>
          </v-list>
        </v-card>
      </v-col>
    </v-row>
    <v-row v-else-if="user_type === 'A'">
      <v-col cols="4" v-for="(user, index) in filteredUsers" :key="user.id" :class="{ 'mb-4': (index + 1) % 3!== 0 }">
        <v-card @click="userClicked(user)">
          <v-card-title>
            <h2>{{ user.nome }}</h2>
          </v-card-title>
          <v-list>
            <v-list-item>
              <v-list-item-title>Specializzazione: {{ user.specializzazione }}</v-list-item-title>
            </v-list-item>
          </v-list>
        </v-card>
      </v-col>
    </v-row>
    <v-dialog v-model="showDetails" max-width="500">
      <v-card>
        <v-card-title>
          {{ selectedUser.denominazione_azienda || selectedUser.nome }}
        </v-card-title>
        <v-list>
          <v-list-item v-if="user_type === 'D'">
            <v-list-item-title>Numero REA: {{ selectedUser.numero_rea }}</v-list-item-title>
          </v-list-item>
          <v-list-item v-if="user_type === 'D'">
            <v-list-item-title>Provincia: {{ selectedUser.provincia }}</v-list-item-title>
          </v-list-item>
          <v-list-item v-if="user_type === 'D'">
            <v-list-item-title>Codice Fiscale: {{ selectedUser.codice_fiscale }}</v-list-item-title>
          </v-list-item>
          <v-list-item v-if="user_type === 'D'">
            <v-list-item-title>Forma Giuridica: {{ selectedUser.forma_giuridica }}</v-list-item-title>
          </v-list-item>
          <v-list-item v-if="user_type === 'D'">
            <v-list-item-title>Descrizione Attività: {{ selectedUser.descrizione_attivita }}</v-list-item-title>
          </v-list-item>
          <v-list-item v-if="user_type === 'A'">
            <v-list-item-title>Indirizzo Studio: {{ selectedUser.indirizzo_studio }}</v-list-item-title>
          </v-list-item>
          <v-list-item v-if="user_type === 'A'">
            <v-list-item-title>Voto Maturità: {{ selectedUser.voto_maturita }}</v-list-item-title>
          </v-list-item>
          <v-list-item v-if="user_type === 'A'">
            <v-list-item-title>Certificazioni Acquisite: {{ selectedUser.certificazioni_acquisite }}</v-list-item-title>
          </v-list-item>
          <v-list-item v-if="user_type === 'A'">
            <v-list-item-title>Esperienze Lavorative: {{ selectedUser.esperienze_lavorative }}</v-list-item-title>
          </v-list-item>
        </v-list>
        <v-card-actions v-if="user_type === 'D'">
          <v-btn color="primary" @click="addCV">Aggiungi CV</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </v-container>
</template>

<script>
import axios from 'axios';
import { jwtDecode } from "jwt-decode";

export default {
  data() {
    return {
      user_type: '',
      search: '',
      users: [],
      showDetails: false,
      selectedUser: {}
    };
  },
  created() {
    const token = localStorage.getItem('token');
    const decodedToken = jwtDecode(token);
    this.user_type = decodedToken.user_type;
    this.fetchUsers();
  },
  methods: {
    async fetchUsers() {
      const token = localStorage.getItem('token');
      try {
        const response = await axios.get(`/api/users`, {
          headers: {
            Authorization: `${token}`
          }
        });
        this.users = response.data;
      } catch (error) {
        console.error(error);
      }
    },
    userClicked(user) {
      this.selectedUser= user;
      this.showDetails = true;
    },
    async sendCV(file) {
      const formData = new FormData();
      formData.append('file', file);
      formData.append('user_id', this.selectedUser.id);

      try {
        const response = await axios.post('/api/cv', formData, {
          headers: {
            'Authorization': localStorage.getItem('token'),
            'Content-Type': 'multipart/form-data'
          }
        });

        if (response.status === 200) {
          alert('CV uploaded successfully');
        } else {
          alert('Failed to upload CV');
        }
      } catch (error) {
        console.error(error);
        alert('Failed to upload CV');
      }
    },
    addCV() {
      const fileInput = document.createElement('input');
      fileInput.type = 'file';
      fileInput.addEventListener('change', (event) => {
        const file = event.target.files[0];
        this.sendCV(file);
      });
      fileInput.click();
    }
  },
  computed: {
    filteredUsers() {
      return this.users.filter(user => {
        if (this.user_type === 'D') {
          return user.provincia.includes(this.search) || user.categoria.includes(this.search);
        } else if (this.user_type === 'A') {
          return user.indirizzo_studio.includes(this.search) || user.voto_maturita.includes(this.search) || user.specializzazione.includes(this.search);
        }
      });
    }
  }
};
</script>

<style scoped>
.search-bar {
  margin-bottom: 20px;
  border-radius: 25px;
  border: none;
  box-shadow: 0 0 10px rgba(0,0,0,0.1);
  padding: 10px 20px;
  font-size: 16px;
  transition: all 0.3s ease;
  width: 100%;
  outline: none;
  background-color: #f5f5f5;
}

.search-bar::placeholder {
  color: #888;
}

.search-bar:focus {
  box-shadow: 0 0 10px rgba(0,123,255,0.5);
  background-color: #fff;
}

.user-search {
  margin: 20px 0;
}

.user-search .v-card {
  margin-bottom: 15px;
  cursor: pointer;
  box-shadow: none;
  border: 1px solid #ddd;
  border-radius: 4px;
  transition: all 0.3s ease;
}

.user-search .v-card-title {
  padding: 10px 16px;
  background-color: #f5f5f5;
  font-weight: bold;
}

.user-search .v-list {
  padding: 0;
}

.user-search .v-list-item {
  padding: 10px 16px;
  border-bottom: 1px solid #eee;
}

.user-search .v-list-item:last-child {
  border-bottom: none;
}

.v-col .v-card .v-card-title {
  font-size: 1.2em;
  font-weight: bold;
  background: rgba(153, 153, 153, 0.75);
}

</style>