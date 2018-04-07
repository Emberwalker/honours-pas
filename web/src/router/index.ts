import Vue from "vue";
import Router, {NavigationGuard} from "vue-router";
import {UserType} from "../lib/Types";

import AdminRoot from "../components/AdminRoot.vue";
import CloneProject from "../components/CloneProject.vue";
import EditProject from "../components/EditProject.vue";
import Login from "../components/Login.vue";
import NewProject from "../components/NewProject.vue";
import NotFound from "../components/NotFound.vue";
import ProjectList from "../components/ProjectList.vue";
import ReportEditor from "../components/ReportEditor.vue";
import ReportView from "../components/ReportView.vue";
import StudentOrderProjects from "../components/StudentOrderProjects.vue";
import StudentPickProjects from "../components/StudentPickProjects.vue";
import store from "../stores";

Vue.use(Router);

const isNotStudentGuard: NavigationGuard = (to, from, next) => {
  if (store.state.user !== null && store.state.user.user_type !== UserType.Student) { return next(); }
  console.warn("Not student bar: " + to.path);
  next("/");
};

const isStudentGuard: NavigationGuard = (to, from, next) => {
  if (store.state.user !== null && store.state.user.user_type === UserType.Student) { return next(); }
  console.warn("Student bar: " + to.path);
  next("/");
};

const isAdminGuard: NavigationGuard = (to, from, next) => {
  if (store.state.user !== null && store.state.user.user_type === UserType.Administrator) {
    return next();
  }
  console.warn("Admin bar: " + to.path);
  next("/");
};

const router = new Router({
  mode: "history",
  routes: [
    {
      beforeEnter: (to, from, next) => {
        if (store.state.user !== null) {
          return next("/");
        }
        next();
      },
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
      beforeEnter: isNotStudentGuard,
      component: NewProject,
      name: "new_project",
      path: "/new",
    },
    {
      beforeEnter: isNotStudentGuard,
      component: EditProject,
      name: "edit_project",
      path: "/edit/:id",
      props: true,
    },
    {
      beforeEnter: isNotStudentGuard,
      component: CloneProject,
      name: "clone_project",
      path: "/clone/:id",
      props: true,
    },
    {
      beforeEnter: isStudentGuard,
      component: StudentPickProjects,
      name: "pick_projects",
      path: "/pick",
    },
    {
      beforeEnter: isStudentGuard,
      component: StudentOrderProjects,
      name: "order_projects",
      path: "/order",
    },
    {
      beforeEnter: isAdminGuard,
      component: AdminRoot,
      name: "admin_root",
      path: "/admin",
    },
    {
      beforeEnter: isAdminGuard,
      component: ReportView,
      name: "generate_report",
      path: "/report/:id",
      props: true,
    },
    {
      beforeEnter: isAdminGuard,
      component: ReportEditor,
      name: "report_editor",
      path: "/report/:id/editor",
      props: true,
    },
    {
      component: NotFound,
      name: "404",
      path: "*",
    },
  ],
});

router.beforeEach((to, from, next) => {
  if (to.path !== "/login" && !store.state.user) {
    return next({ path: "/login", query: { next: to.path } });
  }
  next();
});

export default router;
