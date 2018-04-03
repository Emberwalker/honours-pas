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
              <button @click="generateReport(session)" type="button" class="btn btn-sm btn-primary">Generate Report</button>
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
                  <input type="text" class="form-control form-control-sm" id="new-session-super-name-field" placeholder="Supervisor Name" v-model="newSessionSuperName">
                </div>
                <div class="form-group">
                  <label for="new-session-super-email-field">Supervisor Email</label>
                  <input type="email" autocomplete="email" class="form-control form-control-sm" id="new-session-super-email-field" placeholder="Supervisor Email" v-model="newSessionSuperEmail">
                </div>
                <button hidden="hidden" type="submit"></button>
              </form>
            </div>
            <div class="modal-footer">
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
                Archiving a session will prevent staff from editing any projects in this session and deactivate all
                current student logins, making this session immutable.
              </p>
              <span class="font-weight-bold">
                Are you sure you wish to archive this session?
              </span>
            </div>
            <div class="modal-footer">
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
            <div class="modal-footer">
              <button type="button" data-dismiss="modal" aria-label="Cancel" class="btn btn-sm btn-primary">Cancel</button>
              <button type="button" @click="onPurgeSubmit" data-dismiss="modal" class="float-md-right btn btn-sm btn-danger">Delete Permanently</button>
            </div>
          </div>
        </div>
      </div>
      <!-- Sessions END, Staff START -->
      <div class="row row-break">
        <div class="col">
          <h2 class="h1">
            Staff
            <button type="button" class="btn btn-sm btn-primary ml-2" @click="openStaffEdit(null)">New Staff</button>
          </h2>
        </div>
      </div>
      <div class="row">
        <div v-if="!staffLoaded" class="col">
          <feather :spin="true" icon="refresh-cw"/>
          <span class="h5 text-muted loading-text">Loading...</span>
        </div>
        <div v-else class="col">
          <table class="table table-hover">
            <thead>
              <tr>
                <th scope="col">ID</th>
                <th scope="col">Name</th>
                <th scope="col">Email</th>
                <th scope="col">Is Admin?</th>
                <th scope="col">Actions</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="s in staff" :key="s.id">
                <th scope="row">{{s.id}}</th>
                <td>{{s.full_name}}</td>
                <td>{{s.email}}</td>
                <td v-if="s.is_admin" class="text-weight-bold">&#10003;</td>
                <td v-else class="text-weight-bold">&#10007;</td>
                <td>
                  <button @click="openStaffEdit(s)" type="button" class="btn btn-sm btn-primary">Edit</button>
                  <button @click="onToggleAdmin(s)" :disabled="currentUser(s)" type="button" class="btn btn-sm btn-warning stripes-sm">Toggle Admin</button>
                  <button @click="onRmStaff(s.id)" type="button" class="btn btn-sm btn-danger stripes-sm">Delete</button>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
      <div class="row">
        <div class="col">
          <h3>Upload New Staff</h3>
          <p class="h5 font-weight-normal">Any entries which match an email address that already exists will be updated, not duplicated.</p>
        </div>
      </div>
      <div class="row">
        <div class="col">
          <form @submit.prevent="onNewStaff()">
            <div class="form-group">
              <label for="staffInputFile">CSV File</label>
              <input type="file" class="form-control h-auto" id="staffInputFile" aria-describedby="staffFileHelp" ref="staffInputFile" accept=".csv,text/csv">
              <small id="staffFileHelp">Please only have two columns - email address and full name</small>
            </div>
            <div class="form-check">
              <input type="checkbox" class="form-check-input" id="staffInputHasHeaders" v-model="staffHasHeaders">
              <label for="staffInputHasHeaders">Has Header Row? (If checked, this will drop the first line of input.)</label>
            </div>
            <button type="submit" class="btn btn-sm btn-primary">Upload</button>
          </form>
        </div>
      </div>
      <!-- New/Edit staff modal -->
      <div class="modal fade" ref="staffModal" id="staffModal" tabindex="-1" role="dialog" aria-labelledby="staffModalLabel" aria-hidden="true">
        <div class="modal-dialog" role="document">
          <div class="modal-content">
            <div class="modal-header bg-primary text-white">
              <h5 class="modal-title" id="staffModalLabel">New/Edit Staff</h5>
              <button type="button" class="close" data-dismiss="modal" aria-label="Cancel">
                <span aria-hidden="true">&times;</span>
              </button>
            </div>
            <div class="modal-body">
              <div class="alert alert-warning" role="alert">
                Using an existing email address will <i>update</i> the existing staff member.
              </div>
              <div class="alert alert-danger" role="alert" :hidden="!editStaffInputError">
                One of the fields below is unspecified. Fill all fields and try again.
              </div>
              <form @submit.prevent="onStaffEdit()">
                <div class="form-group">
                  <label for="new-staff-email-field">Staff Email</label>
                  <input type="email" autocomplete="email" class="form-control form-control-sm" id="new-session-super-email-field" placeholder="Staff Email" v-model="staffEditEmail">
                </div>
                <div class="form-group">
                  <label for="new-session-name-field">Full Name</label>
                  <input type="text" class="form-control form-control-sm" id="new-session-name-field" placeholder="Name" v-model="staffEditName">
                </div>
                <button hidden="hidden" type="submit"></button>
              </form>
            </div>
            <div class="modal-footer">
              <button type="button" data-dismiss="modal" aria-label="Cancel" class="btn btn-sm btn-primary">Cancel</button>
              <button type="button" @click="onStaffEdit" class="float-md-right btn btn-sm btn-success">Create/Edit</button>
            </div>
          </div>
        </div>
      </div>
      <!-- Staff END, Students START -->
      <div class="row row-break">
        <div class="col">
          <h2 class="h1">
            Students
            <button :disabled="!hasCurrentSession" type="button" class="btn btn-sm btn-primary ml-2" @click="openStudentEdit(null)">New Student</button>
          </h2>
          <p class="h4 text-muted">
            Current Session
          </p>
          <div class="alert alert-danger" role="alert" :hidden="hasCurrentSession">
            There must be an active session to upload or add students. Create a new session to begin.
          </div>
        </div>
      </div>
      <div class="row">
        <div v-if="!studentsLoaded" class="col">
          <feather :spin="true" icon="refresh-cw"/>
          <span class="h5 text-muted loading-text">Loading...</span>
        </div>
        <div v-else class="col">
          <table class="table table-hover">
            <thead>
              <tr>
                <th scope="col">ID</th>
                <th scope="col">Name</th>
                <th scope="col">Email</th>
                <th scope="col">Actions</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="s in students" :key="s.id">
                <th scope="row">{{s.id}}</th>
                <td>{{s.full_name}}</td>
                <td>{{s.email}}</td>
                <td>
                  <button @click="openStudentEdit(s)" type="button" class="btn btn-sm btn-primary">Edit</button>
                  <button @click="onRmStudent(s.id)" type="button" class="btn btn-sm btn-danger stripes-sm">Delete</button>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
      <div class="row">
        <div class="col">
          <h3>Upload New Students</h3>
          <p class="h5 font-weight-normal">Any entries which match an email address that already exists will be updated, not duplicated.</p>
        </div>
      </div>
      <div class="row">
        <div class="col">
          <form @submit.prevent="onNewStudents()">
            <div class="form-group">
              <label for="studentsInputFile">CSV File</label>
              <input :disabled="!hasCurrentSession" type="file" class="form-control h-auto" id="studentsInputFile" aria-describedby="studentsFileHelp" ref="studentInputFile" accept=".csv,text/csv">
              <small id="studentsFileHelp">Please only have two columns - email address and full name</small>
            </div>
            <div class="form-check">
              <input :disabled="!hasCurrentSession" type="checkbox" class="form-check-input" id="studentsInputHasHeaders" v-model="studentsHasHeaders">
              <label for="studentsInputHasHeaders">Has Header Row? (If checked, this will drop the first line of input.)</label>
            </div>
            <button :disabled="!hasCurrentSession" type="submit" class="btn btn-sm btn-primary">Upload</button>
          </form>
        </div>
      </div>
      <!-- New/Edit student modal -->
      <div class="modal fade" ref="studentModal" id="studentModal" tabindex="-1" role="dialog" aria-labelledby="studentModalLabel" aria-hidden="true">
        <div class="modal-dialog" role="document">
          <div class="modal-content">
            <div class="modal-header bg-primary text-white">
              <h5 class="modal-title" id="studentModalLabel">New/Edit Student</h5>
              <button type="button" class="close" data-dismiss="modal" aria-label="Cancel">
                <span aria-hidden="true">&times;</span>
              </button>
            </div>
            <div class="modal-body">
              <div class="alert alert-warning" role="alert">
                Using an existing email address will <i>update</i> the existing staff member.
              </div>
              <div class="alert alert-danger" role="alert" :hidden="!editStudentInputError">
                One of the fields below is unspecified. Fill all fields and try again.
              </div>
              <form @submit.prevent="onStudentEdit()">
                <div class="form-group">
                  <label for="new-staff-email-field">Student Email</label>
                  <input type="email" autocomplete="email" class="form-control form-control-sm" id="new-session-super-email-field" placeholder="Student Email" v-model="studentEditEmail">
                </div>
                <div class="form-group">
                  <label for="new-session-name-field">Full Name</label>
                  <input type="text" class="form-control form-control-sm" id="new-session-name-field" placeholder="Name" v-model="studentEditName">
                </div>
                <button hidden="hidden" type="submit"></button>
              </form>
            </div>
            <div class="modal-footer">
              <button type="button" data-dismiss="modal" aria-label="Cancel" class="btn btn-sm btn-primary">Cancel</button>
              <button type="button" @click="onStudentEdit" class="float-md-right btn btn-sm btn-success">Create/Edit</button>
            </div>
          </div>
        </div>
      </div>
      <!-- Students END -->
    </div>
  </div>
</template>

<script lang="ts">
import $ from "jquery";
import _ from "lodash";
import Papa from "papaparse";
import Vue from "vue";
import {mapState} from "vuex";
import Actions from "../lib/Actions";
import HTTP from "../lib/HTTP";
import {INewSession, IProject, ISession, ISupervisorCounter, IUserEntry} from "../lib/Types";
import { COMMIT_NOT_WORKING, COMMIT_WORKING } from "../stores/index";

export default Vue.extend({
  computed: {
    admins(): string[] {
      const admins = this.staff.filter((it) => it.is_admin);
      return admins.map((it) => it.email);
    },
    hasCurrentSession(): boolean {
      return _.some(this.$store.state.available_sessions, (it: ISession) => it.is_current);
    },
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
        const sessSorted = _.sortBy(_.values(currSess), (it) => [1 / it.count, it.name]);
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
      editStaffInputError: false,
      editStudentInputError: false,
      newSessionInputError: false,
      newSessionName: "",
      newSessionSuperEmail: this.$store.state.user.email.slice(0), // Force a copy
      newSessionSuperName: this.$store.state.user.name.slice(0),
      staff: [] as IUserEntry[],
      staffEditEmail: "",
      staffEditName: "",
      staffHasHeaders: false,
      staffLoaded: false,
      studentEditEmail: "",
      studentEditName: "",
      students: [] as IUserEntry[],
      studentsHasHeaders: false,
      studentsLoaded: false,
    };
  },
  methods: {
    currentUser(ent: IUserEntry): boolean {
      return ent.email === this.$store.state.user.email;
    },
    generateReport(session: ISession) {
      // TODO
      console.error("Report requested; not implemented! Session:", session.name);
    },
    onArchiveSubmit() {
      if (this.activeModalSession === "") { return; }
      this.$store.dispatch({
        session: parseInt(this.activeModalSession, 10),
        type: Actions.ARCHIVE_SESSION,
      }).then(() => {
        // Update the student list after (as the current session has changed)
        this.updateStudents();
      });
    },
    onPurgeSubmit() {
      if (this.activeModalSession === "") { return; }
      this.$store.dispatch({
        session: parseInt(this.activeModalSession, 10),
        type: Actions.PURGE_SESSION,
      }).then(() => {
        // Update the student list after (as the current session has changed)
        this.updateStudents();
      });
    },
    onRmStaff(id: number) {
      this.$store.commit(COMMIT_WORKING);
      // TODO: Error handling
      HTTP.delete("/staff/" + id).then((res) => {
        this.updateStaff();
      }).finally(() => {
        this.$store.commit(COMMIT_NOT_WORKING);
      });
    },
    onRmStudent(id: number) {
      this.$store.commit(COMMIT_WORKING);
      // TODO: Error handling
      HTTP.delete("/students/" + id).then((res) => {
        this.updateStudents();
      }).finally(() => {
        this.$store.commit(COMMIT_NOT_WORKING);
      });
    },
    onStaffEdit() {
      if (this.staffEditEmail === "" || this.staffEditName === "") {
        this.editStaffInputError = true;
        return;
      }

      const staffer = {
        email: this.staffEditEmail,
        full_name: this.staffEditName,
        is_admin: _.find(this.admins, (it) => it === this.staffEditEmail) !== undefined,
      };
      ($(this.$refs.staffModal) as any).modal("hide");
      this.writeStaff([staffer]);
    },
    onStudentEdit() {
      if (this.studentEditEmail === "" || this.studentEditName === "") {
        this.editStudentInputError = true;
        return;
      }

      const student = {
        email: this.studentEditEmail,
        full_name: this.studentEditName,
      };
      ($(this.$refs.studentModal) as any).modal("hide");
      this.writeStudents([student]);
    },
    onToggleAdmin(entry: IUserEntry) {
      this.$store.commit(COMMIT_WORKING);
      const modified = $.extend({}, entry) as IUserEntry;
      modified.is_admin = !modified.is_admin;
      this.writeStaff([modified]);
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
      }).then(() => {
        // Update the student list after (as the current session has changed)
        this.updateStudents();
      });
    },
    onNewStaff() {
      const input = ($(this.$refs.staffInputFile) as any)[0];
      if (input.files.length === 0) {
        alert("No file provided. Please select a CSV file to upload first.");
        return;
      }
      this.$store.commit(COMMIT_WORKING);
      const file = input.files[0];
      Papa.parse(file, {
        complete: this.onNewStaffCallback,
        delimiter: ",",
        skipEmptyLines: true,
      });
    },
    onNewStaffCallback(parsed: Papa.ParseResult) {
      const raws = this.verifyParse(this.staffHasHeaders, parsed);
      if (!raws) { return; }

      // Remove blank entries (Excel leaves these if a line is deleted)
      _.remove(raws, (it) => it[0] === "" || it[1] === "");

      const built = raws.map((row) => {
        return {
          email: row[0],
          full_name: row[1],
          is_admin: _.find(this.admins, (it) => it === row[0]) !== undefined,
        };
      });

      this.writeStaff(built);
    },
    onNewStudents() {
      const input = ($(this.$refs.studentInputFile) as any)[0];
      if (input.files.length === 0) {
        alert("No file provided. Please select a CSV file to upload first.");
        return;
      }
      this.$store.commit(COMMIT_WORKING);
      const file = input.files[0];
      Papa.parse(file, {
        complete: this.onNewStudentsCallback,
        delimiter: ",",
        skipEmptyLines: true,
      });
    },
    onNewStudentsCallback(parsed: Papa.ParseResult) {
      const raws = this.verifyParse(this.studentsHasHeaders, parsed);
      if (!raws) { return; }

      // Remove blank entries (Excel leaves these if a line is deleted)
      _.remove(raws, (it) => it[0] === "" || it[1] === "");

      const built = raws.map((row) => {
        return {
          email: row[0],
          full_name: row[1],
        };
      });

      this.writeStudents(built);
    },
    openStaffEdit(s: IUserEntry | null) {
      this.editStaffInputError = false;
      if (s !== null) {
        this.staffEditEmail = s.email;
        this.staffEditName = s.full_name;
      } else {
        this.staffEditEmail = "";
        this.staffEditName = "";
      }

      ($(this.$refs.staffModal) as any).modal("show");
    },
    openStudentEdit(s: IUserEntry | null) {
      this.editStudentInputError = false;
      if (s !== null) {
        this.studentEditEmail = s.email;
        this.studentEditName = s.full_name;
      } else {
        this.studentEditEmail = "";
        this.studentEditName = "";
      }

      ($(this.$refs.studentModal) as any).modal("show");
    },
    updateStaff() {
      // TODO: Error handling
      this.staffLoaded = false;
      HTTP.get("/staff").then((res) => {
        this.staff = _.sortBy(res.data.staff as IUserEntry[], "id");
        this.staffLoaded = true;
      });
    },
    updateStudents() {
      // TODO: Error handling
      this.studentsLoaded = false;
      HTTP.get("/students/current").then((res) => {
        this.students = _.sortBy(res.data.students as IUserEntry[], "id");
        this.studentsLoaded = true;
      });
    },
    verifyParse(dropFirst: boolean, parsed: Papa.ParseResult): string[] | null {
      if (parsed.errors.length !== 0) {
        this.$store.commit(COMMIT_NOT_WORKING);
        parsed.errors.forEach((it) => {
          console.error(it);
        });
        alert(parsed.errors.length + " errors occured during parsing.");
        return null;
      }

      let raws = parsed.data;
      if (dropFirst) {
        raws = _.drop(raws, 1); // Drop header
      }
      return raws;
    },
    writeStaff(staff: Array<{full_name: string, email: string, is_admin?: boolean}>) {
      // TODO: Error handling
      HTTP.post("/staff", { staff }).then((res) => {
        this.updateStaff();
      }).finally(() => {
        this.$store.commit(COMMIT_NOT_WORKING);
      });
    },
    writeStudents(students: Array<{full_name: string, email: string}>) {
      // TODO: Error handling
      HTTP.post("/students", { students }).then((res) => {
        this.updateStudents();
      }).finally(() => {
        this.$store.commit(COMMIT_NOT_WORKING);
      });
    },
  },
  mounted() {
    const evtHandler = (evt: any) => {
      this.activeModalSession = $(evt.relatedTarget).data("session");
    };
    $(this.$refs.archiveModal).on("show.bs.modal", evtHandler);
    $(this.$refs.purgeModal).on("show.bs.modal", evtHandler);

    // Kick off loading of staff and students
    this.updateStaff();
    this.updateStudents();
  },
  name: "AdminRoot",
});
</script>

<style scoped lang="scss">
.modal-footer {
  display: block;
}

.btn-new-session {
  margin-left: 1rem;
}

.loading-text {
  vertical-align: top;
}

.row-break {
  margin-top: 2rem;
}

.h-auto {
  height: auto;
}
</style>
