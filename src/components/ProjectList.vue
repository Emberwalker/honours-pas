<template>
<div id="projects-list">
  <h1 class="display-4">
    Projects List
    <button v-if="isStudent" type="button" class="btn-select-projects btn btn-lg btn-primary">Select Projects...</button>
    <router-link v-if="!isStudent" to="/new">
      <button type="button" class="btn-select-projects btn btn-lg btn-primary">New Project...</button>
    </router-link>
  </h1>
  <div class="projlist-section" v-if="hasMarked">
    <h2>Marked Projects</h2>
    <project-card v-for="project in marked" :project="project" :key="project.id">
      <!-- Add extra buttons for bottom of the card here. -->
      <button v-on:click="unmark(project)" type="button" class="btn btn-sm btn-warning">Unmark</button>
    </project-card>
  </div> <!-- if hasMarked END -->
  <div class="projlist-section">
    <h2>Projects by Session</h2>
    <div class="projlist-section" v-for="session in sessions">
      <h3>{{ session.name }}</h3>
      <project-card v-for="project in session.projects" :project="project" :key="project.id">
        <!-- Add extra buttons for bottom of the card here. -->
        <div v-if="session.is_current">
          <button v-if="session.is_current" type="button" class="btn btn-sm btn-primary">Edit</button>
          <button v-if="isMarked(project)" v-on:click="unmark(project)" type="button" class="btn btn-sm btn-warning">Unmark</button>
          <button v-else v-on:click="mark(project)" type="button" class="btn btn-sm btn-primary">Mark</button>
        </div>
      </project-card>
    </div>
  </div>
</div>
</template>

<script lang="ts">
import _ from "lodash";
import Vue from "vue";
import Mutations from "../lib/Mutations";
import { IProject, ISession, UserType } from "../lib/Types";
import ProjectCard from "./ProjectCard.vue";

export default Vue.extend({
  components: {
    "project-card": ProjectCard,
  },
  data() {
    return {};
  },
  computed: {
    sessions(): ISession[] {
      return this.$store.getters.sessions_for_user;
    },
    hasMarked(): boolean {
      const usr = this.$store.state.user;
      if (usr) {
        return usr.marked_projects.length !== 0;
      } else {
        return false;
      }
    },
    marked(): IProject[] {
      const usr = this.$store.state.user;
      const session = this.$store.getters.current_session;
      if (usr && session) {
        return session.projects.filter((it: IProject) => _.includes(usr.marked_projects, it.id));
      } else {
        return [];
      }
    },
    isStudent(): boolean {
      const usr = this.$store.state.user;
      if (usr) {
        return usr.type === UserType.Student;
      }
      return false;
    },
  },
  methods: {
    isMarked(project: IProject): boolean {
      if (!this.$store.state.user) {
        return false;
      }
      return _.includes(this.$store.state.user.marked_projects, project.id);
    },
    mark(project: IProject) {
      this.$store.commit({
        project: project.id,
        type: Mutations.ADD_MARKED_PROJECT,
      });
    },
    unmark(project: IProject) {
      this.$store.commit({
        project: project.id,
        type: Mutations.RM_MARKED_PROJECT,
      });
    },
  },
  name: "ProjectList",
});
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="scss">

.projlist-section {
  margin-top: 1.5rem;
}

// Pad out the button for select projects a little.
.btn-select-projects {
  margin-left: 1rem;
}

</style>
