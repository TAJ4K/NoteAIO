import { createRouter, createWebHistory, RouteRecordRaw } from "vue-router";
import HomeView from "../views/HomeView.vue";

const routes: Array<RouteRecordRaw> = [
  {
    path: "/",
    name: "home",
    component: HomeView,
  },
  {
    path: "/notes",
    name: "notes",
    component: () => import("../views/NotesView.vue"),
  },
  {
    path: "/schedule",
    name: "schedule",
    component: () => import("../views/ScheduleView.vue"),
  },
  {
    path: "/settings",
    name: "settings",
    component: () => import("../views/SettingsView.vue"),
  },
  {
    path: "/notes/:id",
    name: "folder",
    props: true,
    component: () => import("../views/pages/FolderView.vue"),
  },
];

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes,
});

export default router;
