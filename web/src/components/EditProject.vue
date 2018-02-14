<template>
<div class="new-project">
  <h1>Edit Project</h1>
  <project-editor :initial-project="project" :allow-author-changes="isAdmin" v-on:edit-complete="editComplete"/>
</div>
</template>

<script lang="ts">
import _ from "lodash";
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
      this.$store.dispatch({
        project,
        type: Actions.EDIT_PROJECT,
      }).then(() => {
        this.$router.push("/");
      });
    },
  },
  name: "NewProject",
  props: [ "id" ],
});
</script>

<style scoped lang="scss">
</style>
