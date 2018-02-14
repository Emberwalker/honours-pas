import "bootstrap";
import $ from "jquery";
import Vue from "vue";
import App from "./App.vue";
import Feather from "./components/Feather.vue";
import { highlightingInit, renderCodeBlock } from "./lib/Util";
import router from "./router";
import store from "./stores";

Vue.config.productionTip = false;

Vue.component("feather", Feather);

Vue.directive("markdown", {
  componentUpdated: (el) => {
    $(el).find("code").each((idx, inner) => { renderCodeBlock(inner); });
  },
});

const vm = new Vue({
  components: { App },
  el: "#app",
  router,
  store,
  template: "<App/>",
});

highlightingInit();

// Auth guard
router.beforeEach((to, from, next) => {
  if (vm.$store.state.demo_mode) {
    return next(); // SKIP!
  }
  if (to.name !== "login") {
    if (vm.$data.user) {
      return next();
    } else {
      return next("/login");
    }
  } else {
    if (vm.$data.user) {
      return next("/");
    } else {
      return next();
    }
  }
});
