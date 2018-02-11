<template>
  <div class="clone-project">
    <h1>Clone Project</h1>
    <project-editor :initial-project="project" :allow-author-changes="isAdmin" v-on:edit-complete="editComplete"></project-editor>
  </div>
</template>

<script lang="ts">
  import _ from "lodash";
  import Vue from "vue";
  import Mutations from "../lib/Mutations";
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
      const usr = this.$store.state.user;
      let project: IProject | null = null;
      if (session) {
        project = _.find(session.projects, (proj) => proj.id.toString() === this.id);
      }
      project = $.extend({}, project);
      project.id = -1;
      if (usr.user_type !== UserType.Administrator) {
        project.supervisor_name = usr.name;
        project.supervisor_email = usr.email;
      }
      return {
        project,
      };
    },
    methods: {
      editComplete(project: IProject) {
        this.$store.commit({
          project,
          type: Mutations.NEW_PROJECT,
        });
        this.$router.push("/");
      },
    },
    name: "CloneProject",
    props: [ "id" ],
  });
</script>

<style scoped lang="scss">
</style>
