<template>
  <div id="app">
    <nav-header/>
    <div class="container">
      <router-view/>
    </div>
    <div id="app-working-overlay" v-if="isWaiting">
      <span id="app-working-spinner"><feather spin="true" large="true" icon="refresh-cw"/></span>
    </div>
  </div>
</template>

<script lang="ts">
  import Vue from "vue";
  import Feather from "./components/Feather.vue";
  import NavHeader from "./components/NavHeader.vue";

  export default Vue.extend({
    components: {
      "feather": Feather,
      "nav-header": NavHeader,
    },
    computed: {
      isWaiting(): boolean {
        return this.$store.state.working;
      },
    },
    name: "app",
  });
</script>

<style lang="scss">
  @import "scss/custom";

  #app {
    margin-top: 4rem;
  }

  #app-working-overlay {
    background: white;
    opacity: 0.5;
    position: fixed;
    width: 100%;
    height: 100%;
    padding: 0;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
  }

  #app-working-spinner {
    position: relative;
    top: 50%;
    left: 50%;
  }
</style>
