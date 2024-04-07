import { createRouter, createWebHistory } from 'vue-router'
import UserRegistration from './components/UserRegistration.vue'
import UserLogin from './components/UserLogin.vue'

const routes = [
    { path: '/reg', component: UserRegistration },
    { path: '/log', component: UserLogin },
]

const router = createRouter({
    history: createWebHistory(),
    routes,
})

export default router