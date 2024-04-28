<template>
  <v-app-bar app style="background-color: #007BFF;" dark>
    <v-toolbar-title class="text-h5">Diplomatiziati</v-toolbar-title>
    <v-spacer></v-spacer>
    <v-btn to="/profile" class="nav-button">
      <v-icon left class="nav-icon">mdi-account</v-icon>
      Profile
    </v-btn>
    <v-btn to="/search" class="nav-button">
      <v-icon left class="nav-icon">mdi-magnify</v-icon>
      Search
    </v-btn>
  </v-app-bar>
</template>

<script>
export default {
  name: 'NavBar'
}
</script>

<style scoped>
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