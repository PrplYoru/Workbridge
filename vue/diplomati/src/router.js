import { createRouter, createWebHistory } from 'vue-router'
import UserRegistration from './components/UserRegistration.vue'
import UserLogin from './components/UserLogin.vue'
import UserDetails from './components/UserDetails.vue'
import UserProfile from "@/components/UserProfile.vue";

const routes = [
    { path: '/register', component: UserRegistration },
    { path: '/login', component: UserLogin },
    { path: '/details', component: UserDetails },
    { path: '/profile', component: UserProfile },
    { path: '/', redirect: '/login' },
]

const router = createRouter({
    history: createWebHistory(),
    routes,
})

export default router