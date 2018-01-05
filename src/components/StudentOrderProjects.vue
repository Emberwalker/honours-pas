<template>
  <div class="container" id="project-selection">
    <div class="row">
      <div class="col-12" id="projects-list">
        <h1 class="display-4">
          Order Projects
        </h1>
        <p class="h4 text-muted">
          Toggle the equals button between projects to state that two projects are equal.
        </p>
        <div class="container projlist-section">
          <project-card :project="p0" :key="p0.id">
            <!-- Add extra buttons for bottom of the card here. -->
            <button type="button" class="btn btn-sm btn-primary" v-on:click="swap(0)"><feather nopad icon="arrow-down"/></button>
          </project-card>
          <div class="row justify-content-center">
            <button type="button" class="btn" v-on:click="toggleFirstSecond" :class="equalFirstSecond ? 'btn-success' : 'btn-danger'">
              {{ equalFirstSecond ? '=' : '=/=' }}
            </button>
          </div>
          <project-card :project="p1" :key="p1.id">
            <!-- Add extra buttons for bottom of the card here. -->
            <button type="button" class="btn btn-sm btn-primary" v-on:click="swap(0)"><feather nopad icon="arrow-up"/></button>
            <button type="button" class="btn btn-sm btn-primary" v-on:click="swap(1)"><feather nopad icon="arrow-down"/></button>
          </project-card>
          <div class="row justify-content-center">
            <button type="button" class="btn" v-on:click="toggleSecondThird" :class="equalSecondThird ? 'btn-success' : 'btn-danger'">
              {{ equalSecondThird ? '=' : '=/=' }}
            </button>
          </div>
          <project-card :project="p2" :key="p2.id">
            <!-- Add extra buttons for bottom of the card here. -->
            <button type="button" class="btn btn-sm btn-primary" v-on:click="swap(1)"><feather nopad icon="arrow-up"/></button>
          </project-card>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
  import $ from "jquery";
  import _ from "lodash";
  import Vue from "vue";
  import Mutations from "../lib/Mutations";
  import {IProject, IProjectSelection, ISession} from "../lib/Types";
  import Router from "../router";
  import Store from "../stores";
  import ProjectCard from "./ProjectCard.vue";

  export default Vue.extend({
    beforeRouteEnter(to, from, next) {
      if (!Store.state.user || Store.state!.user!.selected_projects!.length !== 3) {
        Router.replace("/pick");
      }
      next();
    },
    components: {
      "project-card": ProjectCard,
    },
    data() {
      let selections: IProjectSelection[] = [];
      _.each(this.$store.state.user.selected_projects, (sel: IProjectSelection) => {
        const selCopy: IProjectSelection = $.extend({}, sel) as any;
        if (selCopy) { selections.push(selCopy); }
      });
      selections = _.sortBy(selections, ["weight"]);
      return {
        equalFirstSecond: false,
        equalSecondThird: false,
        selections,
      };
    },
    computed: {
      p0(): IProject | undefined {
        if (this.selections.length !== 3) { return undefined; }
        return this.getProject(this.selections[0]);
      },
      p1(): IProject | undefined {
        if (this.selections.length !== 3) { return undefined; }
        return this.getProject(this.selections[1]);
      },
      p2(): IProject | undefined {
        if (this.selections.length !== 3) { return undefined; }
        return this.getProject(this.selections[2]);
      },
    },
    methods: {
      getProject(sel: IProjectSelection): IProject | undefined {
        if (!sel) { return undefined; }
        const session: ISession = this.$store.getters.current_session;
        return _.find(session.projects, { id: sel.project });
      },
      toggleFirstSecond() {
        this.equalFirstSecond = !this.equalFirstSecond;
      },
      toggleSecondThird() {
        this.equalSecondThird = !this.equalSecondThird;
      },
      swap(fst: number) {
        // From https://stackoverflow.com/a/41857928
        this.selections.splice(fst, 2, this.selections[fst + 1], this.selections[fst]);
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

</style>
