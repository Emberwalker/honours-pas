<template>
  <div class="admin-root">
    <h1 class="display-4">Administration</h1>
    <div class="container">
      <!-- Sessions START -->
      <div class="row">
        <div class="col">
          <h2 class="h1">
            Sessions
            <button type="button" class="btn-new-session btn btn-lg btn-primary" data-toggle="modal" data-target="#newModal">New Session...</button>
          </h2>
        </div>
      </div>
      <div v-for="session in this.sessions" :key="session.id" class="row">
        <div class="col">
          <div class="card session-card">
            <h3 class="card-header text-white" :class="session.is_current ? 'bg-primary' : 'bg-warning stripes'">
                {{ session.name }} <span v-if="!session.is_current" class="h3 archive-txt text-white float-md-right text-uppercase">Archived</span>
            </h3>
            <div class="card-body">
              <div class="container">
                <div class="row">
                  <div class="col-sm-3">
                    <h4>Total Projects</h4>
                    <p>{{ session.projects.length }}</p>
                  </div>
                  <div class="col-sm-9">
                    <h4>Projects by Supervisor</h4>
                    <ul class="list-unstyled">
                      <li v-for="supervisor in projectsBySupervisor[session.name]" :key="supervisor.email">
                        {{ supervisor.count }}: {{ supervisor.name }} &lt;{{ supervisor.email }}&gt;
                      </li>
                    </ul>
                  </div>
                </div>
              </div>
              <button v-if="session.is_current" @click="generateReport(session)" type="button" class="btn btn-sm btn-primary">Generate Report</button>
              <button v-if="session.is_current" type="button" data-toggle="modal" data-target="#archiveModal" :data-session="session.id" class="float-md-right btn btn-sm btn-danger stripes-sm">Archive Session</button>
              <button v-else type="button" data-toggle="modal" data-target="#purgeModal" :data-session="session.id" class="float-md-right btn btn-sm btn-danger stripes-sm">Delete Permanently</button>
            </div>
          </div>
        </div>
      </div>
      <!-- Sessions modals -->
      <div class="modal fade" ref="newModal" id="newModal" tabindex="-1" role="dialog" aria-labelledby="newModalLabel" aria-hidden="true">
        <div class="modal-dialog" role="document">
          <div class="modal-content">
            <div class="modal-header bg-primary text-white">
              <h5 class="modal-title" id="newModalLabel">New Session</h5>
              <button type="button" class="close" data-dismiss="modal" aria-label="Cancel">
                <span aria-hidden="true">&times;</span>
              </button>
            </div>
            <div class="modal-body">
              <div class="alert alert-warning" role="alert">
                Creating a new session will archive the current session if one exists!
              </div>
              <div class="alert alert-danger" role="alert" :hidden="!newSessionInputError">
                One of the fields below is unspecified. Fill all fields and try again.
              </div>
              <form @submit.prevent="onNewSubmit()">
                <div class="form-group">
                  <label for="new-session-name-field">Session Name</label>
                  <input type="text" class="form-control form-control-sm" id="new-session-name-field" placeholder="Session" v-model="newSessionName">
                </div>
                <div class="form-group">
                  <label for="new-session-super-name-field">Supervisor Name</label>
                  <input type="text" class="form-control form-control-sm" id="new-session-super-name-field" placeholder="Supeervisor Name" v-model="newSessionSuperName">
                </div>
                <div class="form-group">
                  <label for="new-session-super-email-field">Supervisor Email</label>
                  <input type="email" autocomplete="email" class="form-control form-control-sm" id="new-session-super-email-field" placeholder="Supeervisor Email" v-model="newSessionSuperEmail">
                </div>
                <button hidden="hidden" type="submit"></button>
              </form>
            </div>
            <div class="modal-footer arcpurge-footer">
              <button type="button" data-dismiss="modal" aria-label="Cancel" class="btn btn-sm btn-primary">Cancel</button>
              <button type="button" @click="onNewSubmit" class="float-md-right btn btn-sm btn-success">Create</button>
            </div>
          </div>
        </div>
      </div>
      <div class="modal fade" ref="archiveModal" id="archiveModal" tabindex="-1" role="dialog" aria-labelledby="archiveModalLabel" aria-hidden="true">
        <div class="modal-dialog" role="document">
          <div class="modal-content">
            <div class="modal-header bg-danger text-white stripes">
              <h5 class="modal-title" id="archiveModalLabel">Confirm Archival</h5>
              <button type="button" class="close" data-dismiss="modal" aria-label="Cancel">
                <span aria-hidden="true">&times;</span>
              </button>
            </div>
            <div class="modal-body">
              <p>
                Archiving a session will destroy all assosciated student data, and prevent staff from editing any
                projects in this session.
              </p>
              <span class="font-weight-bold">
                Are you sure you wish to archive this session?
              </span>
            </div>
            <div class="modal-footer arcpurge-footer">
              <button type="button" data-dismiss="modal" aria-label="Cancel" class="btn btn-sm btn-primary">Cancel</button>
              <button type="button" @click="onArchiveSubmit" data-dismiss="modal" class="float-md-right btn btn-sm btn-danger">Archive</button>
            </div>
          </div>
        </div>
      </div>
      <div class="modal fade" ref="purgeModal" id="purgeModal" tabindex="-1" role="dialog" aria-labelledby="purgeModalLabel" aria-hidden="true">
        <div class="modal-dialog" role="document">
          <div class="modal-content">
            <div class="modal-header bg-danger text-white stripes">
              <h5 class="modal-title" id="purgeModalLabel">Confirm Permanent Deletion</h5>
              <button type="button" class="close" data-dismiss="modal" aria-label="Cancel">
                <span aria-hidden="true">&times;</span>
              </button>
            </div>
            <div class="modal-body">
              <p>
                Permanently deleting a session will prevent staff from accessing and cloning past projects. This is not
                recommended unless the database is showing high load or low storage.
              </p>
              <span class="font-weight-bold">
                Are you sure you wish to delete this session permanently?
              </span>
            </div>
            <div class="modal-footer arcpurge-footer">
              <button type="button" data-dismiss="modal" aria-label="Cancel" class="btn btn-sm btn-primary">Cancel</button>
              <button type="button" @click="onPurgeSubmit" data-dismiss="modal" class="float-md-right btn btn-sm btn-danger">Delete Permanently</button>
            </div>
          </div>
        </div>
      </div>
      <!-- Sessions END -->
      <div class="row">
        <div class="col">
          <h2 class="h1">Staff</h2>
        </div>
      </div>
      <div class="row">
        <!-- TODO: Staff management -->
      </div>
      <div class="row">
        <div class="col">
          <h2 class="h1">Students</h2>
          <p class="h4 text-muted">
            Current Session
          </p>
        </div>
      </div>
      <div class="row">
        <!-- TODO: Student management -->
      </div>
    </div>
  </div>
</template>

<script lang="ts">
  import _ from "lodash";
  import Vue from "vue";
  import {mapState} from "vuex";
  import Actions from "../lib/Actions";
  import {INewSession, IProject, ISession, ISupervisorCounter} from "../lib/Types";

  export default Vue.extend({
    computed: {
      projectsBySupervisor() {
        const sessions: ISession[] = this.$store.getters.sessions_for_user;
        const out = _.map(sessions, (session: ISession) => {
          const currSess: {[key: string]: ISupervisorCounter} = {};
          _.each(session.projects, (proj: IProject) => {
            if (!currSess[proj.supervisor_email]) {
              currSess[proj.supervisor_email] = {
                count: 0,
                email: proj.supervisor_email,
                name: proj.supervisor_name,
              };
            }
            currSess[proj.supervisor_email].count += 1;
          });
          const sessSorted = _.reverse(_.sortBy(_.values(currSess), ["count"]));
          return [session.name, sessSorted];
        });
        return _.fromPairs(out);
      },
      ...mapState({
        sessions: "available_sessions",
      }),
    },
    data() {
      return {
        activeModalSession: "",
        newSessionInputError: false,
        newSessionName: "",
        newSessionSuperEmail: this.$store.state.user.email.slice(0), // Force a copy
        newSessionSuperName: this.$store.state.user.name.slice(0),
      };
    },
    methods: {
      generateReport(session: ISession) {
        // TODO
        console.error("Report requested; not implemented! Session:", session.name);
      },
      onArchiveSubmit() {
        if (this.activeModalSession === "") { return; }
        this.$store.dispatch({
          session: parseInt(this.activeModalSession, 10),
          type: Actions.ARCHIVE_SESSION,
        });
      },
      onPurgeSubmit() {
        if (this.activeModalSession === "") { return; }
        this.$store.dispatch({
          session: parseInt(this.activeModalSession, 10),
          type: Actions.PURGE_SESSION,
        });
      },
      onNewSubmit() {
        if (this.newSessionName === "" || this.newSessionSuperEmail === "" || this.newSessionSuperName === "") {
          this.newSessionInputError = true;
          return;
        }
        this.newSessionInputError = false;
        ($(this.$refs.newModal) as any).modal("hide");

        const newSess: INewSession = {
          name: this.newSessionName.slice(0),
          supervisor_email: this.newSessionSuperEmail.slice(0),
          supervisor_name: this.newSessionSuperName.slice(0),
        };

        this.$store.dispatch({
          session: newSess,
          type: Actions.NEW_SESSION,
        });
      },
    },
    mounted() {
      const evtHandler = (evt: any) => {
        this.activeModalSession = $(evt.relatedTarget).data("session");
      };
      $(this.$refs.archiveModal).on("show.bs.modal", evtHandler);
      $(this.$refs.purgeModal).on("show.bs.modal", evtHandler);
    },
    name: "AdminRoot",
  });
</script>

<style scoped lang="scss">
.arcpurge-footer {
  display: block;
}

.btn-new-session {
  margin-left: 1rem;
}
</style>
