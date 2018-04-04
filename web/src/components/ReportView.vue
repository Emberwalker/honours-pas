<template>
  <div class="report-root">
    <h1 class="display-4">Session Report: {{ this.sessionName }}</h1>
    <h2>
      By Project
      <button type="button" @click="toggleInitials" class="btn btn-sm btn-primary">Toggle ID/Initials</button>
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
import { COMMIT_NOT_WORKING, COMMIT_WORKING } from "../stores/index";

interface ISessionReportRaw {
  session: { name: string, supervisor_name: string, supervisor_email: string };
  by_student: IRawByStudent[];
  by_project: IRawByProject[];
  students: IStudent[];
  projects: IProject[];
  comments: string[number];
}

interface IRawByStudent {
  student: number;
  choices: number[];
  is_eq: boolean[];
}

interface IRawByProject {
  project: number;
  selections: number[];
  is_eq: boolean[];
}

interface IRawStudent {
  id: number;
  email: string;
  full_name: string;
}

interface IStudent extends IRawStudent {
  comment?: string;
}

interface IStudentMap {
  [key: number]: IStudent;
}

interface IStudentProjectChoice extends IStudent {
  isEq: boolean;
}

interface IProject {
  id: number;
  name: string;
  supervisor_name: string;
  supervisor_email: string;
}

interface IProjectMap {
  [key: number]: IProject;
}

interface IProjectRow extends IProject {
  choices: IStudentProjectChoice[][];
}

interface IStudentRow extends IStudent {
  choices: Array<{ project: number, eq_last: boolean }>;
}

export default Vue.extend({
  computed: {
    allStudents(): IStudent[] {
      if (!this.report) { return []; }
      const all = _.map(this.report.students, (it: IRawStudent) => {
        let comment: string | undefined = this.report!.comments[it.id];
        if (comment === "") { comment = undefined; }
        const out = {
          ...it,
          comment,
        } as IStudent;
        return out;
      }) as any as IStudent[];
      return _.sortBy(all, ["id"]);
    },
    choicesRange(): number[] {
      return _.range(0, this.maxChoices);
    },
    maxChoices(): number {
      return _.reduce(this.projectRows, (curr: number, it: IProjectRow) => {
        if (it.choices.length > curr) { return it.choices.length; }
        return curr;
      }, 0);
    },
    projectRows(): IProjectRow[] {
      if (!this.report) { return []; }

      const out = _.map(this.report.by_project, (it: IRawByProject) => {
        return {
          choices: _.map(_.zip(it.selections, it.is_eq), (outerEntry: [number[], boolean[]]) => {
            return _.map(_.zip(outerEntry[0], outerEntry[1]), (entry: [number, boolean]) => {
              const res: IStudentProjectChoice = {
                isEq: entry[1],
                ...this.students[entry[0]],
              };
              return res;
            });
          }),
          ...this.projects[it.project],
        };
      }) as any as IProjectRow[]; // Completely overrule Typescript here - for some reason it gets this totally wrong.

      return _.sortBy(out, ["supervisor", "name", "id"]);
    },
    projects(): IProjectMap {
      if (!this.report) { return {}; }
      return _.fromPairs(_.map(this.report.projects, (it: IProject) => [it.id, it]));
    },
    sessionName(): string {
      if (!this.report) { return "..."; }
      return this.report.session.name;
    },
    students(): IStudentMap {
      return _.fromPairs(_.map(this.allStudents, (it) => [it.id, it]));
    },
    studentRows(): IStudentRow[] {
      if (!this.report) { return []; }

      const out = _.map(this.report.by_student, (it: IRawByStudent) => {
        const clonedEq = it.is_eq.slice(0);
        clonedEq.unshift(false); // Make the arrays equal length for convenience
        const choices = _.map(_.zip(it.choices, clonedEq), (inner: [number, boolean]) => {
          return {
            eq_last: inner[1],
            project: inner[0],
          };
        });
        return {
          choices,
          ...this.students[it.student],
        };
      }) as any as IStudentRow[]; // Completely overrule Typescript here - for some reason it gets this totally wrong.

      return _.sortBy(out, ["id"]);
    },
  },
  data() {
    return {
      renderInitials: false,
      report: undefined as ISessionReportRaw | undefined,
    };
  },
  methods: {
    renderStudentComment(st: IStudentRow): string {
      if (st.comment) {
        return "\"" + st.comment + "\"";
      }
      return "";
    },
    renderStudentChoices(st: IStudentRow): string {
      let out = "";
      st.choices.forEach((choice, idx) => {
        if (idx !== 0) {
          if (choice.eq_last) { out += " = "; } else { out += " > "; }
        }
        out += choice.project;
      });
      return out;
    },
    renderStudentsByProject(sts: IStudentProjectChoice[] | undefined): string {
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
      this.report = res.data as ISessionReportRaw;
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
  /*(watch: {
    report(newVal: ISessionReportRaw | undefined) {
      if (!newVal) { return; }
      // TODO
    },
  },*/
});
</script>

<style scoped lang="scss">
.table-cell-nowrap {
  white-space: nowrap;
}
</style>
