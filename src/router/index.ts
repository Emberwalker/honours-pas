import Vue from "vue";
import Router from "vue-router";

import EditProject from "../components/EditProject.vue";
import Login from "../components/Login.vue";
import NewProject from "../components/NewProject.vue";
import NotFound from "../components/NotFound.vue";
import ProjectList from "../components/ProjectList.vue";
import StudentPickProjects from "../components/StudentPickProjects.vue";

Vue.use(Router);

export default new Router({
  mode: "history",
  routes: [
    {
      component: Login,
      name: "login",
      path: "/login",
    },
    {
      component: ProjectList,
      name: "project_list",
      path: "/",
    },
    {
      component: NewProject,
      name: "new_project",
      path: "/new",
    },
    {
      component: EditProject,
      name: "edit_project",
      path: "/edit/:id",
      props: true,
    },
    {
      component: StudentPickProjects,
      name: "pick_projects",
      path: "/pick",
    },
    {
      component: NotFound,
      name: "404",
      path: "*",
    },
  ],
});
