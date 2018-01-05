<template>
  <div class="container" id="project-selection">
    <div class="row">
      <div class="col-8" id="projects-list">
        <h1 class="display-4">
          Select Projects
        </h1>
        <div class="projlist-section" v-if="hasMarked">
          <h2>Marked Projects</h2>
          <project-card v-for="project in marked" :project="project" :key="project.id">
            <!-- Add extra buttons for bottom of the card here. -->
            <button v-on:click="unmark(project)" type="button" class="btn btn-sm btn-warning">Unmark</button>

            <button v-if="isSelected(project)" v-on:click="unselect(project)" type="button" class="btn btn-sm btn-danger">Deselect</button>
            <button v-else v-on:click="select(project)" type="button" class="btn btn-sm btn-primary">Select</button>
          </project-card>
        </div> <!-- if hasMarked END -->
        <div class="projlist-section">
          <h2>Available Projects</h2>
          <project-card v-for="project in session.projects" :project="project" :key="project.id">
            <!-- Add extra buttons for bottom of the card here. -->
            <button v-if="isMarked(project)" v-on:click="unmark(project)" type="button" class="btn btn-sm btn-warning">Unmark</button>
            <button v-else v-on:click="mark(project)" type="button" class="btn btn-sm btn-primary">Mark</button>

            <button v-if="isSelected(project)" v-on:click="unselect(project)" type="button" class="btn btn-sm btn-danger">Deselect</button>
            <button v-else v-on:click="select(project)" type="button" class="btn btn-sm btn-primary">Select</button>
          </project-card>
        </div>
      </div>
      <div class="col" id="selected-sidebar">
        <div class="card" id="selected-sidebar-card">
          <h2 class="card-header"><feather icon="user-check"/>Selections</h2>
          <div class="card-body">
            <table class="table" v-if="selectedCount !== 0">
              <thead>
                <tr>
                  <th scope="col">#</th>
                  <th scope="col">Project</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="(selection, idx) in selected">
                  <th scope="row">{{idx + 1}}</th>
                  <th>{{selection.name}}</th>
                </tr>
              </tbody>
            </table>
            <p>Selections: {{selectedCount}}/3</p>
            <router-link v-if="isValid" to="/order">
              <button type="button" class="btn btn-sm btn-success">Next</button>
            </router-link>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
  import _ from "lodash";
  import Vue from "vue";
  import Mutations from "../lib/Mutations";
  import {IProject, ISession} from "../lib/Types";
  import ProjectCard from "./ProjectCard.vue";

  export default Vue.extend({
    components: {
      "project-card": ProjectCard,
    },
    data() {
      return {};
    },
    computed: {
      session(): ISession {
        return this.$store.getters.current_session;
      },
      hasMarked(): boolean {
        const usr = this.$store.state.user;
        if (usr) {
          return usr.marked_projects.length !== 0;
        } else {
          return false;
        }
      },
      selectedCount(): number {
        let projs = this.$store.state.user.selected_projects;
        if (!projs) { projs = []; }
        return projs.length;
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
      selected(): IProject[] {
        const projs: IProject[] = [];
        _.each(this.$store.state.user.selected_projects, (sel) => {
          const proj: IProject | undefined = _.first(_.filter(this.session.projects, {id: sel.project}));
          if (proj) { projs.push(proj); }
        });
        return projs;
      },
      isValid(): boolean {
        return this.$store.state.user.selected_projects.length === 3;
      },
    },
    methods: {
      isMarked(project: IProject): boolean {
        if (!this.$store.state.user) {
          return false;
        }
        return _.includes(this.$store.state.user.marked_projects, project.id);
      },
      isSelected(project: IProject): boolean {
        return _.some(this.$store.state.user.selected_projects, {project: project.id});
      },
      mark(project: IProject) {
        this.$store.commit({
          project: project.id,
          type: Mutations.ADD_MARKED_PROJECT,
        });
      },
      select(project: IProject) {
        let projs = this.$store.state.user.selected_projects;
        if (!projs) {
          projs = [];
        }
        // Skip if the project is already selected
        if (_.some(projs, {project: project.id})) { return; }
        projs.push({
          owner: this.$store.state.user,
          project: project.id,
          weight: 0,
        });
        this.$store.commit({
          projects: projs,
          type: Mutations.SET_SELECTED_PROJECTS,
        });
      },
      unmark(project: IProject) {
        this.$store.commit({
          project: project.id,
          type: Mutations.RM_MARKED_PROJECT,
        });
      },
      unselect(project: IProject) {
        let projs = this.$store.state.user.selected_projects;
        if (!projs) { return; }
        projs = _.reject(projs, {project: project.id});
        this.$store.commit({
          projects: projs,
          type: Mutations.SET_SELECTED_PROJECTS,
        });
      },
    },
    name: "ProjectList",
  });
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="scss">

  #selected-sidebar-card {
    top: 5em;
    position: sticky;
  }

  .projlist-section {
    margin-top: 1.5rem;
  }

  // Pad out the button for select projects a little.
  .btn-select-projects {
    margin-left: 1rem;
  }

</style>
