<template>
  <div class="report-root table-responsive-sm" ref="htmlRoot">
    <h1 class="display-4">Session Editor: {{ this.sessionName }}</h1>
    <h2>
      By Project
      <button type="button" @click="toggleInitials" class="btn btn-sm btn-primary">Toggle ID/Initials</button>
    </h2>
    <p class="h6">
      '=' before an ID means this choice is tied with another.
      '*' after an ID means a comment has been left by this student.
      Click on a student ID/initials to toggle them as taking this project.
      Hover over a student to see full name, ID and comment if one exists. Comments may be truncated.
    </p>
    <table class="table table-striped table-hover table-sm">
      <thead>
        <tr>
          <th scope="col" class="table-cell-type">ID</th>
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
          <td v-for="i in choicesRange" :key="i">
            <!-- Only make a newline INSIDE a tag to avoid spacing issues. -->
            <template v-for="entry in renderStudentsByProject(row.choices[i], row)">{{ entry.seperator }}<a data-html="true"
              :data-title="entry.tooltip" @click="toggleChoice(entry.st, row)" :key="entry.tooltip"
              :class="'has-tooltip ' + (entry.isSel ? 'font-weight-bold' : 'text-muted')">{{ entry.text }}</a></template>
          </td>
        </tr>
      </tbody>
    </table>
    <h2>Messages</h2>
    <p class="h6">This section will show any informational messages, warnings or errors from the current selections.</p>
    <table class="table table-sm table-hover">
      <thead>
        <tr>
          <th scope="col" class="table-cell-type">Type</th>
          <th scope="col">Message</th>
          <th scope="col">Project(s)</th>
          <th scope="col">Student(s)</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="msg in messages" :class="'table-' + msgTableClass(msg.type)">
          <th scope="row" class="table-cell-type">
            <a class="has-tooltip" :data-title="msgTypeName(msg.type)"><feather :icon="msgTypeIcon(msg.type)" :align-bottom="true"/></a>
          </th>
          <td>{{ msg.msg }}</td>
          <td>
            <!-- Only make a newline INSIDE a tag to avoid spacing issues. -->
            <template v-for="entry in renderMsgProjects(msg)">{{ entry.seperator }}<a class="has-tooltip"
              data-html="true" :data-title="entry.tooltip" :key="entry.tooltip">{{ entry.text }}</a></template>
          </td>
          <td>
            <!-- Only make a newline INSIDE a tag to avoid spacing issues. -->
            <template v-for="entry in renderMsgStudents(msg)">{{ entry.seperator }}<a class="has-tooltip"
              data-html="true" :data-title="entry.tooltip" :key="entry.tooltip">{{ entry.text }}</a></template>
          </td>
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

interface IRenderedStudent {
  tooltip: string;
  seperator: string;
  st: IFaces.IStudent | null;
  text: string;
}

interface IRenderedProject {
  tooltip: string;
  seperator: string;
  proj: IFaces.IProject | null;
  text: string;
}

interface IStudentChoiceField extends IRenderedStudent {
  isSel: boolean;
}

enum MessageType {
  ERROR = 0,
  WARNING = 1,
  INFO = 2,
}

interface ILogMessage {
  type: MessageType;
  msg: string;
  projects: number[];
  students: number[];
}

interface ISelectionMap {
  [key: string]: number[];
}

export default Vue.extend({
  computed: {
    allStudents(): IFaces.IStudent[] {
      return this.report.students;
    },
    allProjects(): IFaces.IProject[] {
      return this.report.projects;
    },
    choicesRange(): number[] {
      return this.report.choicesRange();
    },
    maxChoices(): number {
      return this.report.maxChoices;
    },
    messages(): ILogMessage[] {
      const msgs = [] as ILogMessage[];
      const seenStudents: { [key: string]: number[] } = {};
      const seenProjects: number[] = [];

      _.forIn(this.selections, (v: number[], k: string) => {
        const proj: IFaces.IProject = this.projects[parseInt(k, 10)];
        if (!proj) { console.error("Unable to get proj: ", k); return; }
        if (v.length > 0) { seenProjects.push(proj.id); }

        if (v.length > 1) {
          msgs.push({
            msg: "Project has multiple students.",
            projects: [proj.id],
            students: v,
            type: MessageType.WARNING,
          });
        }

        v.forEach((st) => {
          const current = seenStudents[st.toString()] || [];
          current.push(proj.id);
          seenStudents[st.toString()] = current;
        });
      });

      _.forIn(seenStudents, (v: number[], k: string) => {
        if (v.length <= 1) { return; }
        msgs.push({
          msg: "Student has multiple projects.",
          projects: v,
          students: [parseInt(k, 10)],
          type: MessageType.ERROR,
        });
      });

      const fullSeenProjects = _.map(seenProjects, (it) => this.projects[it]) as any as IFaces.IProject[];
      const absentProjects = _.difference(this.allProjects, fullSeenProjects);
      if (absentProjects.length !== 0) {
        msgs.push({
          msg: absentProjects.length.toString() + " projects do not have a student.",
          projects: _.map(absentProjects, (it: IFaces.IProject) => it.id),
          students: [],
          type: MessageType.INFO,
        });
      }

      const choiceStudents = _.map(this.studentRows, (it) => this.students[it.id]);
      const noChoiceStudents = _.difference(this.allStudents, choiceStudents);
      if (noChoiceStudents.length !== 0) {
        msgs.push({
          msg: noChoiceStudents.length.toString() + " students have no choices recorded.",
          projects: [],
          students: _.map(noChoiceStudents, (it) => it.id),
          type: MessageType.INFO,
        });
      }

      const fullSeenStudents = _.map(_.keys(seenStudents),
        (it: number) => this.students[it]) as any as IFaces.IStudent[];
      const absentStudents = _.without(_.difference(this.allStudents, fullSeenStudents), ...noChoiceStudents);
      if (absentStudents.length !== 0) {
        msgs.push({
          msg: absentStudents.length.toString() + " students with choices do not have a project.",
          projects: [],
          students: _.map(absentStudents, (it: IFaces.IStudent) => it.id),
          type: MessageType.INFO,
        });
      }

      return _.sortBy(msgs, ["type", "msg"]);
    },
    projects(): IFaces.IProjectMap {
      return this.report.projectMap;
    },
    projectRows(): IFaces.IProjectRow[] {
      return this.report.projectRows;
    },
    report(): IFaces.SessionReport {
      return new IFaces.SessionReport(this.report_raw);
    },
    students(): IFaces.IStudentMap {
      return this.report.studentMap;
    },
    sessionName(): string {
      return this.report.session().name;
    },
    studentRows(): IFaces.IStudentRow[] {
      return this.report.studentRows;
    },
  },
  data() {
    let lStorage: Storage | undefined;
    try {
      lStorage = window.localStorage;
      lStorage.setItem("__storage_check", "ok");
      if (lStorage.getItem("__storage_check") !== "ok") {
        lStorage = undefined;
      }
    } catch (e) {
      // NOOP
    }
    return {
      localStorage: lStorage,
      renderInitials: false,
      report_raw: undefined as IFaces.ISessionReportRaw | undefined,
      selections: {} as ISelectionMap,
    };
  },
  methods: {
    renderStudentsByProject(sts: IFaces.IStudentProjectChoice[] | undefined,
                            pr: IFaces.IProject): IStudentChoiceField[] {
      if (!sts) { return []; }
      return _.map(
        _.zip(_.sortBy(sts, ["id"]), this.renderStudents(sts)),
        ([choice, student]: [IFaces.IStudentProjectChoice, IRenderedStudent]) => {
          let text = student.text;
          if (choice.isEq) { text = "=" + text; }
          if (student.st && student.st.comment) { text += "*"; }
          return {
            ...student,
            isSel: student.st ? _.includes(this.selections[pr.id.toString()], student.st.id) : false,
            text,
          };
        },
      ) as any; // Override Typescript on this one.
    },
    renderStudents(sts: IFaces.IStudent[]): IRenderedStudent[] {
      const outArr = [] as IRenderedStudent[];
      _.sortBy(sts, ["id"]).forEach((st, idx) => {
        if (!st) { return; }
        let seperator = "";
        if (idx !== 0) { seperator = ", "; }

        // Work out the tooltip contents
        let tooltip = "<b>" + st.id + "/" + st.full_name + "</b>";
        if (st.comment) {
          let comment = st.comment;
          if (comment.length > 140) {
            comment = comment.substring(0, 140) + "...";
          }
          tooltip += ": '" + _.escape(comment) + "'";
        }

        let out = "";
        if (this.renderInitials) {
          _.words(st.full_name).forEach((word) => { out += word[0]; });
        } else {
          out += st.id;
        }

        outArr.push({
          seperator,
          st,
          text: out,
          tooltip,
        });
      });
      return outArr;
    },
    renderMsgProjects(msg: ILogMessage): IRenderedProject[] {
      let outArr = [] as IRenderedProject[];
      _.map(msg.projects, (it) => this.projects[it]).forEach((it: IFaces.IProject | undefined, idx) => {
        if (!it) { return; }
        let seperator = "";
        if (idx !== 0) { seperator = ", "; }

        // Work out the tooltip contents
        const tooltip = "<b>" + it.id + "/" + it.name + "</b>" + ": " + it.supervisor_name + " &lt;" +
                        it.supervisor_email + "&gt;";

        outArr.push({
          proj: it,
          seperator,
          text: it.id.toString(),
          tooltip,
        });
      });

      if (outArr.length > 5) {
        const cut = outArr.length - 5;
        outArr = outArr.slice(0, 5);
        outArr.push({
          proj: null,
          seperator: ", ",
          text: "...",
          tooltip: cut.toString() + " entries elided.",
        });
      }

      return outArr;
    },
    renderMsgStudents(msg: ILogMessage): IRenderedStudent[] {
      let students = this.renderStudents(_.map(msg.students, (it) => this.students[it]));

      if (students.length > 5) {
        const cut = students.length - 5;
        students = students.slice(0, 5);
        students.push({
          seperator: ", ",
          st: null,
          text: "...",
          tooltip: cut.toString() + " entries elided.",
        });
      }

      return students;
    },
    msgTypeIcon(mt: MessageType): string {
      switch (mt) {
        case MessageType.INFO: return "message-square";
        case MessageType.WARNING: return "triangle";
        case MessageType.ERROR: return "octagon";
      }
    },
    msgTypeName(mt: MessageType): string {
      switch (mt) {
        case MessageType.INFO: return "Info";
        case MessageType.WARNING: return "Warning";
        case MessageType.ERROR: return "Error";
        default: return "Unknown - Report this as a bug.";
      }
    },
    msgTableClass(mt: MessageType): string {
      switch (mt) {
        case MessageType.INFO: return "primary";
        case MessageType.WARNING: return "warning";
        case MessageType.ERROR: return "danger";
        default: return "dark stripes"; // Shouldn't happen (in theory)
      }
    },
    toggleChoice(st: IFaces.IStudent, pr: IFaces.IProject) {
      if (_.includes(this.selections[pr.id.toString()], st.id)) {
        Vue.set(this.selections, pr.id.toString(), _.filter(this.selections[pr.id], (it) => it !== st.id));
      } else {
        const current = this.selections[pr.id.toString()] || [];
        current.push(st.id);
        Vue.set(this.selections, pr.id.toString(), current);
      }

      const s: IFaces.ISession = this.report.session();
      if (this.localStorage && s.id !== -1) {
        try {
          this.localStorage.setItem(s.id.toString(), JSON.stringify(this.selections));
        } catch (e) {
          console.error("Unable to save to localStorage:", e);
        }
      }
    },
    toggleInitials() {
      this.renderInitials = !this.renderInitials;
      this.$forceUpdate();
    },
  },
  mounted() {
    ($(this.$refs.htmlRoot) as any).tooltip({
      container: this.$refs.htmlRoot,
      selector: '[class*="has-tooltip"]',
    });
    this.$store.commit(COMMIT_WORKING);
    HTTP().get("/sessions/" + this.id + "/report").then((res) => {
      this.report_raw = res.data as IFaces.ISessionReportRaw;
      if (this.localStorage) {
        try {
          const raw = this.localStorage.getItem(this.report_raw.session.id.toString());
          if (!raw) { return; }
          const sels = JSON.parse(raw);
          this.selections = sels;
        } catch (e) {
          console.info("Unable to load state from Local Storage:", e);
        }
      }
    }).finally(() => {
      this.$store.commit(COMMIT_NOT_WORKING);
    });
  },
  name: "ReportEditor",
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

.table-cell-type {
  width: 3rem;
  max-width: 3rem;
}

.table-cell-type > a {
  padding-left: 0.5rem;
}
</style>

<!-- This has to be unscoped to hit the generated tooltip-inner's from Popper.js -->
<style lang="scss">
.tooltip-inner {
  max-width: 25rem;
}
</style>
