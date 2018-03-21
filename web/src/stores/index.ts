import Axios from "axios";
import * as _ from "lodash";
import Vue from "vue";
import Vuex from "vuex";
import Actions from "../lib/Actions";
import HTTP from "../lib/HTTP";
import Mutations from "../lib/Mutations";
import {IProject, IProjectSelection, ISession, IUser, UserType} from "../lib/Types";
// import {Promise} from "es6-promise";

Vue.use(Vuex);

// True for user testing demos, False for real auth/backend connect.
const DEMO_MODE = false;

let initialDemoSessions: ISession[] = [];
let initialUser: IUser | null = null;
if (DEMO_MODE) {
  // tslint:disable:object-literal-sort-keys
  initialDemoSessions = [
    {
      id: 2,
      name: "2017-2018",
      is_current: true,
      supervisor_name: "Prof. I.P. Freely",
      supervisor_email: "i.p.freely@not.dundee.ac.uk",
      projects: [
        {
          name: "Sample Project #1",
          supervisor_name: "Prof. I.P. Freely",
          supervisor_email: "i.p.freely@not.dundee.ac.uk",
          additional_staff: [],
          description_md: `Some sort of _description_ with other elements.

1. Lists!
2. [Links!](https://google.com)
3. Code!

\`\`\`c
int main() {
  return -1;
}
\`\`\``,
          id: 1234,
        },
        {
          name: "Sample Project #2",
          supervisor_name: "Douglas Fargo",
          supervisor_email: "d.fargo@not.dundee.ac.uk",
          additional_staff: ["Zane Donovan <z.donovan@not.dundee.ac.uk>"],
          description_md: "Some kind of project description again...",
          id: 1235,
        },
        {
          name: "Sample Project #3",
          supervisor_name: "Douglas Fargo",
          supervisor_email: "d.fargo@not.dundee.ac.uk",
          additional_staff: ["Zane Donovan <z.donovan@not.dundee.ac.uk>"],
          description_md: "Another project for demonstration purposes.",
          id: 1236,
        },
        {
          name: "Sample Project #4",
          supervisor_name: "Zane Donovan",
          supervisor_email: "z.donovan@not.dundee.ac.uk",
          additional_staff: [],
          description_md: "Yet another project for demonstration purposes.",
          id: 1237,
        },
      ],
    },
    {
      id: 1,
      name: "2016-2017",
      is_current: false,
      supervisor_name: "Prof. I.P. Freely",
      supervisor_email: "i.p.freely@not.dundee.ac.uk",
      projects: [
        {
          name: "Old Sample Project #1",
          supervisor_name: "Prof. I.P. Freely",
          supervisor_email: "i.p.freely@not.dundee.ac.uk",
          additional_staff: [],
          description_md: "Some kind of older project description...",
          id: 1233,
        },
      ],
    },
  ];

  initialUser = {
    name: "Test User",
    email: "test.user@not.dundee.ac.uk",
    marked_projects: [1234],
    selected_projects: [],
    selection_comment: "",
    user_type: UserType.Administrator,
  };
  // tslint:enable:object-literal-sort-keys
}

// Private mutations (only accessible within actions)
enum _Mutations {
  ADD_MARKED_PROJECT = "ADD_MARKED_PROJECT",
  RM_MARKED_PROJECT = "RM_MARKED_PROJECT",
  SET_SELECTED_PROJECTS = "SET_SELECTED_PROJECTS",
  SET_SELECTION_COMMENT = "SET_SELECTION_COMMAND",
  NEW_PROJECT = "NEW_PROJECT",
  EDIT_PROJECT = "EDIT_PROJECT",
  RM_PROJECT = "RM_PROJECT",
  NEW_SESSION = "NEW_SESSION",
  ARCHIVE_SESSION = "ARCHIVE_SESSION",
  PURGE_SESSION = "PURGE_SESSION",
  SET_SERVER_OPS = "SET_SERVER_OPS",
}

export const COMMIT_WORKING = {
  isWorking: true,
  type: Mutations.SET_IS_WORKING,
};

export const COMMIT_NOT_WORKING = {
  isWorking: false,
  type: Mutations.SET_IS_WORKING,
};

export interface IReadableError {
  src: Error | null;
  human: string;
}

let initialServerOpts: any | null = null;
let initialWorking: boolean = true;

if (!DEMO_MODE) {
  HTTP.get("/meta").then((response) => {
    const opts = response.data;
    STORE.commit({
      opts,
      type: _Mutations.SET_SERVER_OPS,
    });
    STORE.commit(COMMIT_NOT_WORKING);
  }).catch((err) => {
    STORE.commit(getErrorCommit("Unable to fetch client metadata. Is the API server running?", err));
  });
} else {
  initialWorking = false;
  initialServerOpts = {
    auth: "simple",
    base_url: "",
  };
}

export function getErrorCommit(human: string, err: Error | null): {type: string, err: IReadableError} {
  return {
    err: {
      human,
      src: err,
    },
    type: Mutations.SET_ERROR,
  };
}

function sleep(ms: number): Promise<any> {
  return new Promise((resolve) => _.delay(resolve, ms));
}

// tslint:disable:object-literal-sort-keys
const STORE = new Vuex.Store({
  getters: {
    current_session: (state) => {
      return _.first(state.available_sessions.filter((val) => val.is_current));
    },
    sessions_for_user: (state, getters) => {
      const user = state.user;
      if (user) {
        if (user.user_type === UserType.Administrator || user.user_type === UserType.Staff) {
          return state.available_sessions;
        } else {
          return [ getters.current_session ];
        }
      } else {
        return [];
      }
    },
  },
  state: {
    available_sessions: initialDemoSessions as ISession[],
    demo_mode: DEMO_MODE as boolean,
    server_opts: initialServerOpts as any | null,
    user: initialUser as IUser | null,
    staged_selections: [] as IProjectSelection[],
    working: initialWorking,
    error: null as IReadableError | null,
  },
  mutations: {
    [_Mutations.ADD_MARKED_PROJECT](state, payload) {
      if (state.user) {
        state.user.marked_projects = _.union([payload.project], state.user.marked_projects);
      }
    },
    [_Mutations.RM_MARKED_PROJECT](state, payload) {
      if (state.user) {
        state.user.marked_projects = _.without(state.user.marked_projects, payload.project);
      }
    },
    [_Mutations.SET_SELECTED_PROJECTS](state, payload) {
      if (state.user) {
        state.user.selected_projects = payload.projects;
      }
    },
    [_Mutations.SET_SELECTION_COMMENT](state, payload) {
      if (state.user) {
        state.user.selection_comment = payload.comment;
      }
    },
    [Mutations.SET_USER](state, payload) {
      state.user = payload.user;
    },
    [_Mutations.NEW_PROJECT](state, payload) {
      const session = _.first(state.available_sessions.filter((val) => val.is_current))!;
      const project = payload.project;
      // If demo mode, generate a local ID
      if (state.demo_mode) {
        let maxId = 0;
        _.forEach(session.projects, (p: IProject) => { if (p.id! > maxId) { maxId = p.id!; } });
        project.id = maxId + 1;
      }
      session.projects.push(payload.project);
    },
    [_Mutations.EDIT_PROJECT](state, payload) {
      const session = _.first(state.available_sessions.filter((val) => val.is_current))!;
      const project = payload.project!;
      const idx = _.findIndex(session.projects, (ent) => ent.id === project.id);
      session.projects[idx] = project;
    },
    [_Mutations.RM_PROJECT](state, payload) {
      const session = _.first(state.available_sessions.filter((val) => val.is_current))!;
      session.projects = _.filter(session.projects, (p: IProject) => {
        return p.id !== payload.project;
      });
    },
    [_Mutations.NEW_SESSION](state, payload) {
      _.each(state.available_sessions, (it) => it.is_current = false);
      state.available_sessions = [payload.session, ...state.available_sessions];
    },
    [_Mutations.ARCHIVE_SESSION](state, payload) {
      const session = _.first(state.available_sessions.filter((val) => val.id === payload.session));
      if (!session) { return; }
      session.is_current = false;
    },
    [_Mutations.PURGE_SESSION](state, payload) {
      state.available_sessions = _.filter(state.available_sessions, (val) => val.id !== payload.session);
    },
    [Mutations.SET_IS_WORKING](state, payload) {
      state.working = payload.isWorking;
    },
    [_Mutations.SET_SERVER_OPS](state, payload) {
      state.server_opts = payload.opts;
    },
    [Mutations.SET_ERROR](state, payload) {
      console.error(payload.err.human, payload.err.err);
      state.error = payload.err;
    },
    [Mutations.SET_PROJECTS_AND_SESSIONS](state, payload) {
      state.available_sessions = payload.sessions;
    },
    [Mutations.SET_STAGED_SELECTIONS](state, payload) {
      state.staged_selections = payload.selections;
    },
  },
  actions: {
    async [Actions.ADD_MARKED_PROJECT](ctx, payload) {
      ctx.commit(COMMIT_WORKING);
      const promise = HTTP.post("/me/marks", { id: payload.project }).then((res) => {
        ctx.commit({
          type: _Mutations.ADD_MARKED_PROJECT,
          project: payload.project,
        });
      });

      promise.finally(() => {
        ctx.commit(COMMIT_NOT_WORKING);
      });

      return promise;
    },
    async [Actions.RM_MARKED_PROJECT](ctx, payload) {
      ctx.commit(COMMIT_WORKING);
      const promise = HTTP.delete("/me/marks/" + payload.project).then((res) => {
        ctx.commit({
          type: _Mutations.RM_MARKED_PROJECT,
          project: payload.project,
        });
      });

      promise.finally(() => {
        ctx.commit(COMMIT_NOT_WORKING);
      });

      return promise;
    },
    async [Actions.SET_SELECTED_PROJECTS](ctx, payload) {
      ctx.commit(COMMIT_WORKING);
      const promise = HTTP.put("/me/selections", { selections: payload.projects }).then((res) => {
        ctx.commit({
          type: _Mutations.SET_SELECTED_PROJECTS,
          projects: payload.projects,
        });
      });

      promise.finally(() => {
        ctx.commit(COMMIT_NOT_WORKING);
      });

      return promise;
    },
    async [Actions.SET_SELECTION_COMMENT](ctx, payload) {
      ctx.commit(COMMIT_WORKING);
      const promise = HTTP.put("/me/comment", { comment: payload.comment }).then((res) => {
        ctx.commit({
          type: _Mutations.SET_SELECTION_COMMENT,
          comment: payload.comment,
        });
      });

      promise.finally(() => {
        ctx.commit(COMMIT_NOT_WORKING);
      });

      return promise;
    },
    async [Actions.NEW_PROJECT](ctx, payload) {
      ctx.commit(COMMIT_WORKING);
      const promise = HTTP.post("/projects", payload.project).then((res) => {
        ctx.commit({
          type: _Mutations.NEW_PROJECT,
          project: res.data,
        });
      });

      promise.finally(() => {
        ctx.commit(COMMIT_NOT_WORKING);
      });

      return promise;
    },
    async [Actions.EDIT_PROJECT](ctx, payload) {
      ctx.commit(COMMIT_WORKING);
      const promise = HTTP.put("/projects/" + payload.project.id, payload.project).then((res) => {
        ctx.commit({
          type: _Mutations.EDIT_PROJECT,
          project: payload.project,
        });
      });

      promise.finally(() => {
        ctx.commit(COMMIT_NOT_WORKING);
      });

      return promise;
    },
    async [Actions.RM_PROJECT](ctx, payload) {
      ctx.commit(COMMIT_WORKING);
      const promise = HTTP.delete("/projects/" + payload.project).then((res) => {
        ctx.commit({
          type: _Mutations.RM_PROJECT,
          project: payload.project,
        });
      });

      promise.finally(() => {
        ctx.commit(COMMIT_NOT_WORKING);
      });

      return promise;
    },
    async [Actions.ARCHIVE_SESSION](ctx, payload) {
      ctx.commit(COMMIT_WORKING);
      const promise = HTTP.post("/sessions/" + payload.session + "/archive", {}).then((res) => {
        ctx.commit({
          type: _Mutations.ARCHIVE_SESSION,
          session: payload.session,
        });
      });

      promise.finally(() => {
        ctx.commit(COMMIT_NOT_WORKING);
      });

      return promise;
    },
    async [Actions.PURGE_SESSION](ctx, payload) {
      ctx.commit(COMMIT_WORKING);
      const promise = HTTP.delete("/sessions/" + payload.session).then((res) => {
        ctx.commit({
          type: _Mutations.PURGE_SESSION,
          session: payload.session,
        });
      });

      promise.finally(() => {
        ctx.commit(COMMIT_NOT_WORKING);
      });

      return promise;
    },
    async [Actions.NEW_SESSION](ctx, payload) {
      ctx.commit(COMMIT_WORKING);
      const promise = HTTP.post("/sessions", payload.session).then((res) => {
        const sess: any = res.data;
        sess.projects = [] as IProject[];
        sess.is_current = true;
        ctx.commit({
          type: _Mutations.NEW_SESSION,
          session: sess as ISession,
        });
      });

      promise.finally(() => {
        ctx.commit(COMMIT_NOT_WORKING);
      });

      return promise;
    },
  },
});
// tslint:enable:object-literal-sort-keys

export default STORE;
