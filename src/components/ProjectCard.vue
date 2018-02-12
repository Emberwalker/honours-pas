<template>
  <div class="card">
    <h3 class="card-header text-white" :class="isCurrent ? 'bg-primary' : 'bg-warning stripes'">
      {{ project.name }} <span v-if="!isCurrent" class="h3 archive-txt text-white float-md-right text-uppercase">Archived</span>
    </h3>
    <div class="card-body">
      <h5 class="text-muted">Supervisor:
        <a :href="mailto">{{project.supervisor_name}}</a>
      </h5>
      <h6 class="text-muted" v-if="project.additional_staff.length > 0">
        Additional staff: {{ additional_staff }}
      </h6>
      <div class="description" ref="description" v-html="description" v-markdown></div>
      <slot></slot>
    </div>
  </div>
</template>

<script lang="ts">
import _ from "lodash";
import Vue from "vue";
import { parseMarkdown, renderCodeBlock } from "../lib/Util";

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
  methods: {
    refreshCodeBlocks() {
      const root = $(this.$refs.description);
      _.each(root.find("code"), (el: any) => {
        renderCodeBlock(el);
      });
    },
  },
  mounted() {
    this.refreshCodeBlocks();
  },
  name: "ProjectCard",
  props: {
    isCurrent: {
      default: true,
      required: false,
      type: Boolean,
    },
    project: {
      required: true,
      type: Object,
    },
  },
  updated() {
    this.refreshCodeBlocks();
  },
});
</script>

<style scoped lang="scss">
</style>
