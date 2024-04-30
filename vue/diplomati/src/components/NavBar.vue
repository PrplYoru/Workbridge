<template>
  <v-app-bar app dark>
    <v-toolbar-title class="text-h5">Workbridge</v-toolbar-title>
    <v-spacer></v-spacer>
    <v-btn v-for="item in items" :key="item.title" :to="item.to" class="nav-button">
      <v-icon left class="nav-icon">{{ item.icon }}</v-icon>
      {{ item.title }}
    </v-btn>
  </v-app-bar>
</template>

<script>
export default {
  name: 'NavBar',
  data() {
    return {
      items: [
        { title: 'Profilo', to: '/profile', icon: 'mdi-account' },
        { title: 'Ricerca', to: '/search', icon: 'mdi-magnify' },
      ],
    };
  },
};
</script>

<style scoped>
.v-app-bar {
  background-color: #007BFF;
}

.v-btn {
  color: white;
  font-weight: bold;
  padding: 10px 20px;
}

.v-toolbar-title {
  color: white;
  font-weight: bold;
}

.nav-button:hover {
  background-color: rgba(255, 255, 255, 0.1);
}

.nav-icon {
  color: #fff;
}
</style>