<template>
  <div class="admin-root">
    <h1 class="display-4">Administration</h1>
    <div class="container">
      <!-- Sessions START -->
      <div class="row">
        <div class="col">
          <h2 class="h1">Sessions</h2>
        </div>
      </div>
      <div v-for="session in this.sessions" class="row">
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
                      <li v-for="supervisor in projectsBySupervisor[session.name]">
                        {{ supervisor.count }}: {{ supervisor.name }} <{{ supervisor.email }}>
                      </li>
                    </ul>
                  </div>
                </div>
              </div>
              <button v-if="session.is_current" type="button" data-toggle="modal" data-target="#archiveModal" :data-session="session.name" class="float-md-right btn btn-sm btn-danger stripes-sm">Archive Session</button>
              <button v-else type="button" data-toggle="modal" data-target="#purgeModal" :data-session="session.name" class="float-md-right btn btn-sm btn-danger stripes-sm">Delete Permanently</button>
            </div>
          </div>
        </div>
      </div>
      <!-- Sessions modals -->
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
  import {IProject, ISession, ISupervisorCounter} from "../lib/Types";

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
      };
    },
    methods: {
      onArchiveSubmit() {
        // TODO
        // tslint:disable:no-console
        console.error("Archival requested; not implemented! Session:", this.activeModalSession);
        // tslint:enable:no-console
      },
      onPurgeSubmit() {
        // TODO
        // tslint:disable:no-console
        console.error("Purge requested; not implemented! Session:", this.activeModalSession);
        // tslint:enable:no-console
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
</style>
