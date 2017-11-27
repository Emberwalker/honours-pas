import "bootstrap";
import * as feather from "feather-icons";
import $ from "jquery";
import Vue from "vue";
import App from "./App.vue";
import { highlightingInit, renderCodeBlock } from "./lib/Util";
import router from "./router";
import store from "./stores";

Vue.config.productionTip = false;

Vue.component("feather", {
  functional: true,
  props: {
    icon: {
      required: true,
      type: String,
    },
  },
  render: (createElement, ctx) => {
    return createElement("span", {
      attrs: {
        class: "feather-icon-svg",
      },
      domProps: {
        innerHTML: feather.icons[ctx.props.icon].toSvg(),
      },
    });
  },
});

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
