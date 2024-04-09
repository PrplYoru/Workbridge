import { createRouter, createWebHistory } from 'vue-router'
import UserRegistration from './components/UserRegistration.vue'
import UserLogin from './components/UserLogin.vue'
import UserDetails from './components/UserDetails.vue' // Import the UserDetails component

const routes = [
    { path: '/register', component: UserRegistration },
    { path: '/login', component: UserLogin },
    { path: '/details', component: UserDetails }, // Add the /details route
]

const router = createRouter({
    history: createWebHistory(),
    routes,
})

export default router