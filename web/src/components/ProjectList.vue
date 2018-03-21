<template>
<div id="projects-list">
  <h1 class="display-4">
    Projects List
    <router-link v-if="isStudent" to="/pick">
      <button type="button" class="btn-select-projects btn btn-lg btn-primary">Select Projects...</button>
    </router-link>
    <router-link v-else to="/new">
      <button type="button" class="btn-select-projects btn btn-lg btn-primary">New Project...</button>
    </router-link>
  </h1>
  <div class="projlist-section" v-if="hasMarked">
    <h2 v-if="isStudent">Marked Projects</h2>
    <h2 v-else>Your Projects</h2>
    <project-card v-for="project in marked" :project="project" :key="project.id">
      <!-- Add extra buttons for bottom of the card here. -->
      <router-link v-if="canEdit(project)" :to="'/edit/' + project.id">
        <button type="button" class="btn btn-sm btn-primary">Edit</button>
      </router-link>
      <router-link v-if="!isStudent" :to="'/clone/' + project.id">
        <button type="button" class="btn btn-sm btn-primary">Clone</button>
      </router-link>
      <button v-if="isStudent" v-on:click="unmark(project)" type="button" class="btn btn-sm btn-warning">Unmark</button>
    </project-card>
  </div> <!-- if hasMarked END -->
  <div class="projlist-section">
    <h2>Projects by Session</h2>
    <div class="projlist-section" v-for="session in sessions" :key="session.id">
      <h3>{{ session.name }}</h3>
      <p class="h5 font-weight-normal text-muted" v-if="!session || session.projects.length === 0">No projects in this session.</p>
      <project-card v-if="session" v-for="project in session.projects" :project="project" :key="project.id" :isCurrent="session.is_current">
        <!-- Add extra buttons for bottom of the card here. -->
        <router-link v-if="session.is_current && canEdit(project)" :to="'/edit/' + project.id">
          <button type="button" class="btn btn-sm btn-primary">Edit</button>
        </router-link>
        <router-link v-if="!isStudent" :to="'/clone/' + project.id">
          <button type="button" class="btn btn-sm btn-primary">Clone</button>
        </router-link>
        <div v-if="isStudent && session.is_current">
          <button v-if="isMarked(project)" v-on:click="unmark(project)" type="button" class="btn btn-sm btn-warning">Unmark</button>
          <button v-else v-on:click="mark(project)" type="button" class="btn btn-sm btn-primary">Mark</button>
        </div>
        <button v-if="session.is_current && isAdmin" type="button" data-toggle="modal" data-target="#rmModal" :data-project="project.id" class="float-md-right btn btn-sm btn-danger stripes-sm">Delete</button>
      </project-card>
    </div>
  </div>
  <!-- Modal for delete -->
  <div class="modal fade" ref="rmModal" id="rmModal" tabindex="-1" role="dialog" aria-labelledby="rmModalLabel" aria-hidden="true">
    <div class="modal-dialog" role="document">
      <div class="modal-content">
        <div class="modal-header bg-danger text-white stripes">
          <h5 class="modal-title" id="rmModalLabel">Confirm Deletion</h5>
          <button type="button" class="close" data-dismiss="modal" aria-label="Cancel">
            <span aria-hidden="true">&times;</span>
          </button>
        </div>
        <div class="modal-body">
          <p class="font-weight-bold">
            {{rmProjectName}}
          </p>
          <p class="subtitle font-weight-normal text-muted">
            Supervisor: {{rmProjectSuper}}
          </p>
          <p>
            Deleting a project will invalidate any student choices dependant on it. Affected students are
            listed below for this project.
          </p>
          <ul>
            <li v-for="student in affectedStudents" :key="student.id">
              <a target="_blank" :href="'mailto:' + student.email">{{student.full_name}} &lt;{{student.email}}&gt;</a>
            </li>
          </ul>
          <span class="font-weight-bold">
            Are you sure you wish to delete this project?
          </span>
        </div>
        <div class="modal-footer rm-footer">
          <button type="button" data-dismiss="modal" aria-label="Cancel" class="btn btn-sm btn-primary">Cancel</button>
          <button type="button" @click="onRmSubmit" data-dismiss="modal" class="float-md-right btn btn-sm btn-danger">Delete</button>
        </div>
      </div>
    </div>
  </div>
</div>
</template>

<script lang="ts">
import _ from "lodash";
import Vue from "vue";
import Actions from "../lib/Actions";
import HTTP from "../lib/HTTP";
import Mutations from "../lib/Mutations";
import { IProject, ISession, UserType } from "../lib/Types";
import { COMMIT_NOT_WORKING, COMMIT_WORKING, getErrorCommit } from "../stores/index";
import ProjectCard from "./ProjectCard.vue";

export default Vue.extend({
  components: {
    "project-card": ProjectCard,
  },
  data() {
    return {
      rmProjectId: -1,
      affectedStudents: [] as any[],
    };
  },
  computed: {
    sessions(): ISession[] {
      return this.$store.getters.sessions_for_user;
    },
    hasMarked(): boolean {
      const usr = this.$store.state.user;
      if (usr.user_type === UserType.Staff || usr.user_type === UserType.Administrator) {
        const session = this.$store.getters.current_session;
        return session && session.projects.filter((it: IProject) =>
          it.supervisor_email === usr.email).length !== 0;
      }
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
        // For staff/admins, show their own projects instead
        if (usr.user_type === UserType.Staff || usr.user_type === UserType.Administrator) {
          return session.projects.filter((it: IProject) => it.supervisor_email === usr.email);
        }
        return session.projects.filter((it: IProject) => _.includes(usr.marked_projects, it.id));
      } else {
        return [];
      }
    },
    rmProjectName(): string {
      const p = _.first(_.filter(this.$store.getters.current_session.projects, (it) => it.id === this.rmProjectId));
      return p ? p.name : "<null>";
    },
    rmProjectSuper(): string {
      const p = _.first(_.filter(this.$store.getters.current_session.projects, (it) => it.id === this.rmProjectId));
      return p ? p.supervisor_name + "<" + p.supervisor_email + ">" : "<null>";
    },
    isAdmin(): boolean {
      const usr = this.$store.state.user;
      if (usr) {
        return usr.user_type === UserType.Administrator;
      }
      return false;
    },
    isStudent(): boolean {
      const usr = this.$store.state.user;
      if (usr) {
        return usr.user_type === UserType.Student;
      }
      return false;
    },
  },
  methods: {
    canEdit(project: IProject): boolean {
      if (this.isStudent) { return false; }
      return (this.isAdmin || project.supervisor_email === this.$store.state.user.email);
    },
    isMarked(project: IProject): boolean {
      if (!this.$store.state.user) {
        return false;
      }
      return _.includes(this.$store.state.user.marked_projects, project.id);
    },
    mark(project: IProject) {
      this.$store.dispatch({
        project: project.id,
        type: Actions.ADD_MARKED_PROJECT,
      });
    },
    onRmSubmit() {
      if (this.rmProjectId === -1) { return; }
      const project = _.head(_.filter(this.$store.getters.current_session.projects, (p: IProject) => {
        return p.id === this.rmProjectId;
      }));
      if (!project) { return; }
      this.$store.dispatch({
        project: project.id,
        type: Actions.RM_PROJECT,
      });
    },
    unmark(project: IProject) {
      this.$store.dispatch({
        project: project.id,
        type: Actions.RM_MARKED_PROJECT,
      });
    },
  },
  mounted() {
    $(this.$refs.rmModal).on("show.bs.modal", (evt: any) => {
      this.rmProjectId = $(evt.relatedTarget).data("project");
      this.$store.commit(COMMIT_WORKING);
      HTTP.get("/projects/" + this.rmProjectId + "/students").then((res) => {
        this.affectedStudents = res.data.students;
      }).catch((e) => {
        this.$store.commit({
          type: Mutations.SET_ERROR,
          error: getErrorCommit("unable to fetch affected students.", e),
        });
      }).finally(() => {
        this.$store.commit(COMMIT_NOT_WORKING);
      });
    });
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

.rm-footer {
  display: block;
}

p.subtitle {
  margin-top: -1rem;
}

</style>
