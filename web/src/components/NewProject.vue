<template>
<div class="new-project">
  <h1>New Project</h1>
  <project-editor :initial-project="project" :allow-author-changes="isAdmin" v-on:edit-complete="editComplete"/>
</div>
</template>

<script lang="ts">
import Vue from "vue";
import Actions from "../lib/Actions";
import {IProject, UserType} from "../lib/Types";
import ProjectEditor from "./ProjectEditor.vue";

export default Vue.extend({
  components: {
    "project-editor": ProjectEditor,
  },
  computed: {
    isAdmin(): boolean {
      return this.$store.state.user && this.$store.state.user.user_type === UserType.Administrator;
    },
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
      this.$store.dispatch({
        project: newProject,
        type: Actions.NEW_PROJECT,
      }).then(() => {
        this.$router.push("/");
      });
    },
  },
  name: "NewProject",
});
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="scss">
</style>
