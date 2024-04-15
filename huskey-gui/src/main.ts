import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import { createRouter, createWebHashHistory } from "vue-router";
import LandingScreen from "@/routes/LandingScreen.vue";
import OpenDatabase from "@/routes/OpenDatabase.vue";
import CreateDatabase from "@/routes/CreateDatabase.vue";
import OpenedDatabase from "@/routes/OpenedDatabase.vue";
import PasswordEdit from "@/components/PasswordEdit.vue";
import { databaseState } from "./store/useDatabase";

const routes = [
  { path: '/', component: LandingScreen },
  { path: '/open', component: OpenDatabase },
  { path: '/create', component: CreateDatabase },
  { path: '/database', component: OpenedDatabase }, 
  { path: '/database/create', component: PasswordEdit },
  { path: '/:pathMatch(.*)*', redirect: '/' }
];

const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

// if on the database ui and the database is not open, redirect to the landing screen
router.beforeEach((to, from, next) => {
  if (to.path.startsWith("/database") && !databaseState.database) {
    next("/");
  } else {
    next();
  }
});

const app = createApp(App);
app.use(router);

app.mount("#app");
