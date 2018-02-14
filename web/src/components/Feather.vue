<template>
  <span class="feather-icon-svg" :class="getClasses()" v-html="getIcon()"></span>
</template>

<script lang="ts">
  import * as feather from "feather-icons";
  import Vue from "vue";

  export default Vue.extend({
    methods: {
      getClasses(): string {
        let out = "";
        if (this.nopad || this.spin) {
          out += " nopad";
        }
        if (this.spin) {
          out += " spinning";
        }
        if (this.large) {
          out += " large";
        }
        return out;
      },
      getIcon(): string {
        return feather.icons[this.icon].toSvg();
      },
    },
    name: "Feather",
    props: {
      icon: {
        required: true,
        type: String,
      },
      large: {
        default: false,
        required: false,
        type: Boolean,
      },
      nopad: {
        default: false,
        required: false,
        type: Boolean,
      },
      spin: {
        default: false,
        required: false,
        type: Boolean,
      },
    },
  });
</script>

<style>

  span.feather-icon-svg.no-pad {
    padding-right: 0;
  }

  span.feather-icon-svg.large > svg {
    width: 4rem;
    height: 4rem;
  }

  span.feather-icon-svg.spinning > svg {
    /*animation-name: spin;
    animation-duration: 4000ms;
    animation-iteration-count: infinite;
    animation-timing-function: linear;*/
    animation: featherspin 1s linear infinite;
    transform-origin: 50% 50%;
    display: inline-block;
  }

  @keyframes featherspin {
    /*from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }*/
    100% {
      transform: rotate(360deg);
    }
  }

</style>
