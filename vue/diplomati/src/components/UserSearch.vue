<template>
  <v-container>
    <v-text-field class="search-bar" v-model="search" label="Search"></v-text-field>
    <v-row>
      <UserCard v-for="(user) in filteredUsers" :key="user.id" :user="user" :userType="user_type" @userClicked="userClicked" />
    </v-row>
    <v-dialog v-model="showDetails" max-width="500">
      <UserDetailsDialog :user="selectedUser" :userType="user_type" @addCV="addCV" />
    </v-dialog>
    <input type="file" ref="fileInput" @change="sendCV" style="display: none" />
  </v-container>
</template>

<script>
import axios from 'axios';
import { jwtDecode } from "jwt-decode";
import UserCard from './UserCard.vue';
import UserDetailsDialog from "@/components/UserDetailsDialog.vue";

export default {
  components: {
    UserDetailsDialog,
    UserCard
  },
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
        console.error('Failed to fetch users:', error);
      }
    },
    userClicked(user) {
      this.selectedUser= user;
      this.showDetails = true;
    },
    async sendCV(event) {
      const file = event.target.files[0];
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
        console.error('Failed to upload CV:', error);
      }
    },
    addCV() {
      this.$refs.fileInput.click();
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

</style>