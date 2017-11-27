<template>
<div class="new-project">
  <h1>New Project</h1>
  <project-editor :initial-project="project" v-on:edit-complete="editComplete"/>
</div>
</template>

<script lang="ts">
import _ from "lodash";
import Vue from "vue";
import { IProject } from "../lib/Types";
import ProjectEditor from "./ProjectEditor.vue";

export default Vue.extend({
  components: {
    "project-editor": ProjectEditor,
  },
  data() {
    return {
      project: {
        additional_staff: [],
        description_md: "",
        name: "Untitled Project",
        supervisor_email: this.$store.state.user.email,
        supervisor_name: this.$store.state.user.name,
      },
    };
  },
  methods: {
    editComplete(newProject: IProject) {
      // TODO
    },
  },
  name: "NewProject",
});
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="scss">
</style>
