<template>
  <div class="report-root table-responsive-sm">
    <h1 class="display-4">Session Report: {{ this.sessionName }}</h1>
    <h2>
      By Project
      <button type="button" @click="toggleInitials" class="btn btn-sm btn-primary">Toggle ID/Initials</button>
      <router-link :to="'/report/' + sessionId + '/editor'" :hidden="!sessionId">
        <button type="button" class="btn btn-sm btn-primary">To Editor...</button>
      </router-link>
    </h2>
    <p class="h6">
      '=' before an ID means this choice is tied with another.
      '*' after an ID means a comment has been left by this student.
    </p>
    <table class="table table-striped table-hover table-sm">
      <thead>
        <tr>
          <th scope="col">ID</th>
          <th scope="col">Project</th>
          <th scope="col">Supervisor</th>
          <th v-for="i in choicesRange" :key="i" scope="col">Choice {{ i + 1 }}</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="row in projectRows" :key="row.id">
          <th scope="row">{{ row.id }}</th>
          <td>{{ row.name }}</td>
          <td><a :href="'mailto:' + row.supervisor_email">{{ row.supervisor_name }}</a></td>
          <td v-for="i in choicesRange">
            <span v-html="renderStudentsByProject(row.choices[i])"></span>
          </td>
        </tr>
      </tbody>
    </table>
    <h2>Student List</h2>
    <table class="table table-striped table-hover table-sm">
      <thead>
        <tr>
          <th scope="col" class="table-cell-nowrap">ID</th>
          <th scope="col" class="table-cell-nowrap">Name</th>
          <th scope="col" class="table-cell-nowrap">Email</th>
          <th scope="col" class="table-cell-nowrap">Choices</th>
          <th scope="col">Comment</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="row in studentRows" :key="row.id">
          <th class="table-cell-nowrap" scope="row">{{ row.id }}</th>
          <td class="table-cell-nowrap">{{ row.full_name }}</td>
          <td class="table-cell-nowrap"><a :href="'mailto:' + row.email">{{ row.email }}</a></td>
          <td class="table-cell-nowrap">{{ renderStudentChoices(row) }}</td>
          <td>{{ renderStudentComment(row) }}</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script lang="ts">
import $ from "jquery";
import _ from "lodash";
import Vue from "vue";
import HTTP from "../lib/HTTP";
import * as IFaces from "../lib/SessionReports";
import { COMMIT_NOT_WORKING, COMMIT_WORKING } from "../stores/index";

export default Vue.extend({
  computed: {
    choicesRange(): number[] {
      return this.report.choicesRange();
    },
    maxChoices(): number {
      return this.report.maxChoices;
    },
    projectRows(): IFaces.IProjectRow[] {
      return this.report.projectRows;
    },
    report(): IFaces.SessionReport {
      return new IFaces.SessionReport(this.report_raw);
    },
    sessionId(): number {
      return this.report.session().id;
    },
    sessionName(): string {
      return this.report.session().name;
    },
    studentRows(): IFaces.IStudentRow[] {
      return this.report.studentRows;
    },
  },
  data() {
    return {
      renderInitials: false,
      report_raw: undefined as IFaces.ISessionReportRaw | undefined,
    };
  },
  methods: {
    renderStudentComment(st: IFaces.IStudentRow): string {
      if (st.comment) {
        return "\"" + _.escape(st.comment) + "\"";
      }
      return "";
    },
    renderStudentChoices(st: IFaces.IStudentRow): string {
      let out = "";
      st.choices.forEach((choice, idx) => {
        if (idx !== 0) {
          if (choice.eq_last) { out += " = "; } else { out += " > "; }
        }
        out += choice.project;
      });
      return out;
    },
    renderStudentsByProject(sts: IFaces.IStudentProjectChoice[] | undefined): string {
      if (!sts) { return ""; }
      let out = "";
      _.sortBy(sts, ["id"]).forEach((st, idx) => {
        if (idx !== 0) { out += ", "; }
        if (st.isEq) { out += "="; }
        if (this.renderInitials) {
          out += "<abbr title=\"" + st.id + "\">";
          _.words(st.full_name).forEach((word) => { out += word[0]; });
          out += "</abbr>";
        } else {
          out += st.id;
        }
        if (st.comment) { out += "*"; }
      });
      return out;
    },
    toggleInitials() {
      this.renderInitials = !this.renderInitials;
      this.$forceUpdate();
    },
  },
  mounted() {
    this.$store.commit(COMMIT_WORKING);
    HTTP().get("/sessions/" + this.id + "/report").then((res) => {
      this.report_raw = res.data as IFaces.ISessionReportRaw;
    }).finally(() => {
      this.$store.commit(COMMIT_NOT_WORKING);
    });
  },
  name: "ReportView",
  props: {
      id: {
          required: true,
          type: String,
      },
  },
});
</script>

<style scoped lang="scss">
.table-cell-nowrap {
  white-space: nowrap;
}
</style>
