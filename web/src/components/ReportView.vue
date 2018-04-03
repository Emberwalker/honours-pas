<template>
  <div class="report-root">
    <!-- TODO -->
  </div>
</template>

<script lang="ts">
import $ from "jquery";
import _ from "lodash";
import Vue from "vue";
import { mapState } from "vuex";
import HTTP from "../lib/HTTP";
import { IProject, ISession, IUserEntry } from "../lib/Types";
import { COMMIT_NOT_WORKING, COMMIT_WORKING } from "../stores/index";

export default Vue.extend({
  computed: {
      // TODO
  },
  data() {
    return {
      // TODO
    };
  },
  methods: {
    // TODO
  },
  mounted() {
    // TODO
  },
  name: "ReportView",
  props: {
      id: {
          type: Number,
          required: true,
      },
  },
});
</script>

<style scoped lang="scss">
</style>
