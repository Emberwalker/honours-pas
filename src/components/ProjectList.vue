<template>
<div id="projects-list">
  <h1 class="display-4">Projects List</h1>
  <div class="projlist-section" v-if="hasMarked">
    <h2>Marked Projects</h2>
    <project-card v-for="project in marked" :project="project">
      <!-- Add extra buttons for bottom of the card here. -->
      <!--<button type="button" class="btn btn-primary">Thing</button>-->
    </project-card>
  </div> <!-- if hasMarked END -->
  <div class="projlist-section">
    <h2>Projects by Session</h2>
    <div class="projlist-section" v-for="session in sessions">
      <h3>{{ session.name }}</h3>
      <project-card v-for="project in session.projects" :project="project">
        <!-- Add extra buttons for bottom of the card here. -->
      </project-card>
    </div>
  </div>
</div>
</template>

<script lang="ts">
import * as _ from "lodash";
import ProjectCard from "./ProjectCard.vue";
import { IProject, ISession } from "../lib/Types";
import Vue from "vue";

export default Vue.extend({
  name: "ProjectList",
  components: {
    'project-card': ProjectCard,
  },
  data () {
    return {}
  },
  computed: {
    sessions (): ISession[] {
      return this.$store.getters.sessions_for_user
    },
    hasMarked (): boolean {
      const usr = this.$store.state.user;
      if (usr) {
        return usr.marked_projects.length != 0;
      } else {
        return false;
      }
    },
    marked (): IProject[] {
      const usr = this.$store.state.user;
      const session = this.$store.getters.current_session
      if (usr && session) {
        return session.projects.filter((it: IProject) => _.includes(usr.marked_projects, it.id));
      } else {
        return [];
      }
    }
  }
});
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="scss">

.projlist-section {
  margin-top: 1.5rem;
}

</style>
