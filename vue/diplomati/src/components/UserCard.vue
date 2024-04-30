<template>
  <v-col cols="4" :class="{ 'mb-4': (index + 1) % 3!== 0 }">
    <v-card @click="$emit('userClicked', user)">
      <v-card-title>
        <h2>{{ userTitle }}</h2>
      </v-card-title>
      <v-list>
        <v-list-item>
          <v-list-item-title>{{ userSubtitle }}</v-list-item-title>
        </v-list-item>
      </v-list>
    </v-card>
  </v-col>
</template>

<script>
export default {
  props: ['user', 'userType'],
  computed: {
    userTitle() {
      return this.userType === 'D' ? this.user.denominazione_azienda : this.user.nome;
    },
    userSubtitle() {
      return this.userType === 'D' ? `Categoria: ${this.user.categoria}` : `Specializzazione: ${this.user.specializzazione}`;
    }
  }
};
</script>

<style scoped>
.v-card {
  margin-bottom: 15px;
  cursor: pointer;
  box-shadow: 0 4px 6px 0 hsla(0, 0%, 0%, 0.07);
  border: none;
  border-radius: 10px;
  transition: all 0.3s ease;
  background-color: #ffffff;
  padding: 20px;
  font-family: 'Roboto', sans-serif;
}

.v-card:hover {
  transform: scale(1.03);
  box-shadow: 0 4px 20px 0 hsla(0, 0%, 0%, 0.12);
}

.v-card-title {
  padding: 10px 0;
  color: #333333;
  font-weight: 500;
  font-size: 1.2em;
}

.v-list-item {
  padding: 10px 0;
  color: #666666;
  border-bottom: 1px solid #eeeeee;
}

.v-col .v-card .v-card-title {
  font-size: 1.2em;
  font-weight: bold;
  color: #333333;
}

</style>