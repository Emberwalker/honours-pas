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
          <div class="row">
            <div class="col-1 align-self-center rank-text">1.</div>
            <div class="col">
              <project-card :project="p0" :key="p0.id">
                <!-- Add extra buttons for bottom of the card here. -->
                <button type="button" class="btn btn-sm btn-primary" v-on:click="swap(0)"><feather nopad icon="arrow-down"/></button>
              </project-card>
            </div>
          </div>
          <div class="row justify-content-center">
            <button type="button" class="btn" v-on:click="toggleFirstSecond" :class="equalFirstSecond ? 'btn-success' : 'btn-danger'">
              {{ equalFirstSecond ? '==' : '=/=' }}
            </button>
          </div>
          <div class="row">
            <div class="col-1 align-self-center rank-text">2.</div>
            <div class="col">
              <project-card :project="p1" :key="p1.id">
                <!-- Add extra buttons for bottom of the card here. -->
                <button type="button" class="btn btn-sm btn-primary" v-on:click="swap(0)"><feather nopad icon="arrow-up"/></button>
                <button type="button" class="btn btn-sm btn-primary" v-on:click="swap(1)"><feather nopad icon="arrow-down"/></button>
              </project-card>
            </div>
          </div>
          <div class="row justify-content-center">
            <button type="button" class="btn" v-on:click="toggleSecondThird" :class="equalSecondThird ? 'btn-success' : 'btn-danger'">
              {{ equalSecondThird ? '==' : '=/=' }}
            </button>
          </div>
          <div class="row">
            <div class="col-1 align-self-center rank-text">3.</div>
            <div class="col">
              <project-card :project="p2" :key="p2.id">
                <!-- Add extra buttons for bottom of the card here. -->
                <button type="button" class="btn btn-sm btn-primary" v-on:click="swap(1)"><feather nopad icon="arrow-up"/></button>
              </project-card>
            </div>
          </div>
        </div>
      </div>
    </div>
    <div class="row">
      <div class="col-12">
        <h1 class="h2 section-header">
          Selection Comment
        </h1>
        <p class="h4 text-muted">
          An optional comment for the module coordinator about your selections.
        </p>
        <textarea title="selection comments" v-model="comment" rows="2" class="form-control"></textarea>
        <button type="submit" class=" submit-btn btn btn-lg btn-success" @click="submit">Submit</button>
      </div>
    </div>
    <!-- Finished Modal -->
    <div class="modal fade" ref="doneModal" id="doneModal" tabindex="-1" role="dialog" aria-labelledby="doneModalLabel" aria-hidden="true">
      <div class="modal-dialog" role="document">
        <div class="modal-content">
          <div class="modal-header bg-primary text-white">
            <h5 class="modal-title" id="doneModalLabel">Finished</h5>
            <button type="button" class="close" data-dismiss="modal" aria-label="Cancel">
              <span aria-hidden="true">&times;</span>
            </button>
          </div>
          <div class="modal-body">
            <p><b>Your choices have been submitted.</b> You can now close this window.</p>
            <p>If you need to change the order or comment on your selection, close this dialog and resubmit.</p>
            <p>If you want to change any of your selections entirely, return to the home page by clicking 'Honours Projects' in the navigation bar.</p>
          </div>
          <div class="modal-footer">
            <button type="button" data-dismiss="modal" aria-label="Cancel" class="btn btn-sm btn-primary">Close</button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
  import $ from "jquery";
  import _ from "lodash";
  import Vue from "vue";
  import Actions from "../lib/Actions";
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
      _.each(this.$store.state.staged_selections, (sel: IProjectSelection) => {
        const selCopy: IProjectSelection = $.extend({}, sel) as any;
        if (selCopy) { selections.push(selCopy); }
      });
      selections = _.sortBy(selections, ["weight"]);
      return {
        comment: "",
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
      submit() {
        let weight = 9;
        this.selections[0].weight = weight;
        if (!this.equalFirstSecond) { weight -= 4; }
        this.selections[1].weight = weight;
        if (!this.equalSecondThird) { weight -= 4; }
        this.selections[2].weight = weight;
        this.$store.dispatch({
          projects: this.selections,
          type: Actions.SET_SELECTED_PROJECTS,
        }).then(() => {
          return this.$store.dispatch({
            comment: this.comment,
            type: Actions.SET_SELECTION_COMMENT,
          });
        }).then(() => {
          ($(this.$refs.doneModal) as any).modal("show");
        });
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

  .section-header {
    margin-top: 1.5rem;
  }

  .submit-btn {
    margin-top: 1.5rem;
  }

  .rank-text {
    font-size: 3rem;
  }

</style>
