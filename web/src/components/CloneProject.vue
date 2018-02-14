<template>
  <div class="clone-project">
    <h1>Clone Project</h1>
    <project-editor :initial-project="project" :allow-author-changes="isAdmin" v-on:edit-complete="editComplete"></project-editor>
  </div>
</template>

<script lang="ts">
  import _ from "lodash";
  import Vue from "vue";
  import Actions from "../lib/Actions";
  import {IProject, ISession, UserType} from "../lib/Types";
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
      const usr = this.$store.state.user;
      let project: IProject | null = null;
      _.each(this.$store.state.available_sessions, (session: ISession) => {
        const found = _.find(session.projects, (proj: IProject) => {
          return proj.id!.toString() === this.id;
        });
        if (found) { project = found; }
      });
      if (!project) {
        return { project: null };
      }
      // Note we have to use 'project!' to work around a TS compiler issue
      project = $.extend({}, project);
      project!.id = -1;
      if (usr.user_type !== UserType.Administrator) {
        project!.supervisor_name = usr.name;
        project!.supervisor_email = usr.email;
      }
      return {
        project,
      };
    },
    methods: {
      editComplete(project: IProject) {
        this.$store.dispatch({
          project,
          type: Actions.NEW_PROJECT,
        }).then(() => {
          this.$router.push("/");
        });
      },
    },
    name: "CloneProject",
    props: [ "id" ],
  });
</script>

<style scoped lang="scss">
</style>
