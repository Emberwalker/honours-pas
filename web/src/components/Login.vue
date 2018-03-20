<template>
</template>

<script lang="ts">
import Vue from "vue";

export default Vue.extend({
  name: "Login",
  computed: {
    server_opts() {
      return this.$store.state.server_opts;
    },
  },
  data() {
    return {
      show_form: false,
    };
  },
  methods: {
    update_opts: () => {}
  },
  mounted() {
    if (!this.$store.state.server_opts) {
      return;
    }
    if (this.$store.state.server_opts.auth == "oauth2") {

    }
  },
  watch: {

  },
});
</script>

<style scoped lang="scss">
</style>
