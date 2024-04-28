<template>
  <div id="app" :class="{'with-navbar': !showNavbar, 'without-navbar': showNavbar}">
    <v-app>
      <NavBar v-if="showNavbar"/>
      <router-view/>
    </v-app>
  </div>
</template>

<script>
import NavBar from "@/components/NavBar.vue";

export default {
  name: 'App',
  components: {
    NavBar
  },
  computed: {
    showNavbar() {
      const pathsWithoutNavbar = ['/register', '/login', '/details'];
      return !pathsWithoutNavbar.includes(this.$route.path);
    }
  }
}
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
}

.with-navbar {
  margin-top: 0px;
}

.without-navbar {
  margin-top: 60px;
}
</style>