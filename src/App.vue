<template>
  <div id="app">
    <nav-header/>
    <div class="container">
      <router-view/>
    </div>
  </div>
</template>

<script lang="ts">
  import NavHeader from './components/NavHeader.vue'

  export default {
    name: 'app',
    components: {
      'nav-header': NavHeader
    }
  }
</script>

<style lang="scss">
  @import "scss/custom";

  #app {
    margin-top: 4rem;
  }
</style>
