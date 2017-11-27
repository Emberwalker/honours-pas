<template>
  <div class="card">
    <h3 class="card-header">{{ project.name }}</h3>
    <div class="card-body">
      <h5 class="text-muted">Supervisor:
        <a :href="mailto">{{project.supervisor_name}}</a>
      </h5>
      <h6 class="text-muted" v-if="project.additional_staff.length > 0">
        Additional staff: {{ additional_staff }}
      </h6>
      <div class="description" v-html="description" v-markdown></div>
      <slot></slot>
    </div>
  </div>
</template>

<script lang="ts">
import _ from "lodash";
import Vue from "vue";
import { parseMarkdown } from "../lib/Util";

export default Vue.extend({
  computed: {
    description(): string {
      return parseMarkdown(this.project.description_md);
    },
    mailto(): string {
      return "mailto:" + this.project.supervisor_email;
    },
    additional_staff(): string {
      return _.join(this.project.additional_staff, ", ");
    },
  },
  data() {
    return {};
  },
  name: "ProjectCard",
  props: [
    "project",
  ],
});
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="scss">
</style>
