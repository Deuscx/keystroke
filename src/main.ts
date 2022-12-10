import { createApp } from "vue";
import "./style.css";

import 'uno.css'
import App from "./App.vue";
import { createRouter, createWebHistory } from 'vue-router'
import routes from '~pages'

const router = createRouter({
    routes,
    history: createWebHistory()
})
createApp(App).use(router).mount("#app");
