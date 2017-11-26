import "bootstrap";
import Vue from "vue";
import App from "./App.vue";
import router from "./router";
import store from "./stores";
import $ from "jquery";
import { highlightingInit, renderCodeBlock } from "./lib/Util"

Vue.config.productionTip = false;

const vm = new Vue({
  components: { App },
  el: "#app",
  router,
  store,
  template: "<App/>",
});

Vue.directive('markdown', {
  componentUpdated: el => {
    $(el).find("code").each((_idx, el) => { renderCodeBlock(el); })
  }
})

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
