import { createApp } from "vue";
import { createRouter, createWebHistory } from "vue-router"
import "./styles.scss";
import App from "./App.vue"
import Home from "./views/Home.vue"

const router = createRouter({
    history: createWebHistory(),
    routes: [
        {
            path: '/',
            component: Home
        },
        {
            path: '/settings',
            component: () => import("./views/Settings.vue")
        }
    ]})

const app = createApp(App)

app.use(router).mount("#app");
