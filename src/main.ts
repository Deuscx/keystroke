import { createApp } from 'vue'
import '@unocss/reset/tailwind.css'
import 'uno.css'
import './styles.css'
import routes from 'virtual:generated-pages'
import { createRouter, createWebHistory } from 'vue-router'
import App from './App.vue'

const app = createApp(App)
const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
})
app.use(router)
app.mount('#app')
