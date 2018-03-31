<template>
  <nav class="navbar navbar-expand-md navbar-dark bg-dark fixed-top">
    <div class="container">
      <router-link class="navbar-brand" to="/" exact>Honours Projects</router-link>
      <button class="navbar-toggler" type="button" data-toggle="collapse" data-target="#navbar-collapse" aria-controls="navbar-collapse" aria-expanded="false" aria-label="Toggle navigation">
        <span class="navbar-toggler-icon"></span>
      </button>
      <div id="navbar-collapse" class="collapse navbar-collapse">
        <ul class="navbar-nav mr-auto">
          <li v-if="isAdmin" class="nav-item">
            <router-link class="nav-link" to="/admin">Administration</router-link>
          </li>
        </ul>
        <a v-if="isLoggedIn" href="/api/v1/logout" class="text-white"><feather alt="logout" icon="log-out"/></a>
      </div>
    </div>
  </nav>
</template>

<script lang="ts">
  import Vue from "vue";
  import {UserType} from "../lib/Types";

  export default Vue.extend({
    computed: {
      isAdmin(): boolean {
        return this.$store.state.user && this.$store.state.user.user_type === UserType.Administrator;
      },
      isLoggedIn(): boolean {
        return true && this.$store.state.user;
      },
    },
    data() {
      return {};
    },
    name: "NavHeader",
  });
</script>

<style scoped lang="scss">

  .navbar-brand {
    // Fix screwy padding.
    padding-top: 0;
  }

</style>
