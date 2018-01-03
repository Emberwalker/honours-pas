<template>
<div class="new-project">
  <h1>Edit Project</h1>
  <project-editor :initial-project="project" v-on:edit-complete="editComplete"/>
</div>
</template>

<script lang="ts">
import _ from "lodash";
import Vue from "vue";
import Mutations from "../lib/Mutations";
import { IProject } from "../lib/Types";
import ProjectEditor from "./ProjectEditor.vue";

export default Vue.extend({
  components: {
    "project-editor": ProjectEditor,
  },
  data() {
    const session = this.$store.getters.current_session;
    let project: IProject | null = null;
    if (session) {
      project = _.find(session.projects, (proj) => proj.id.toString() === this.id);
    }
    return {
      project,
    };
  },
  methods: {
    editComplete(project: IProject) {
      this.$store.commit({
        project,
        type: Mutations.EDIT_PROJECT,
      });
      this.$router.push("/");
    },
  },
  name: "NewProject",
  props: [ "id" ],
});
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="scss">
</style>
