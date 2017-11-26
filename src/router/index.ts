import Vue from "vue";
import Router from "vue-router";

import Index from "../components/Index.vue";
import Login from "../components/Login.vue";
import NotFound from "../components/NotFound.vue";
import ProjectList from "../components/ProjectList.vue";

Vue.use(Router);

export default new Router({
  mode: "history",
  routes: [
    {
      component: Index,
      name: "index",
      path: "/",
    },
    {
      component: Login,
      name: "login",
      path: "/login",
    },
    {
      component: ProjectList,
      name: "project_list",
      path: "/list",
    },
    {
      component: NotFound,
      name: "404",
      path: "*",
    },
  ],
});
