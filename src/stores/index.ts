import * as _ from "lodash";
import Vue from "vue";
import Vuex from "vuex";
import Actions from "../lib/Actions";
import Mutations from "../lib/Mutations";
import {IProject, ISession, IUser, UserType} from "../lib/Types";
// import {Promise} from "es6-promise";

Vue.use(Vuex);

// True for user testing demos, False for real auth/backend connect.
const DEMO_MODE = true;

let initialDemoSessions: ISession[] = [];
let initialUser: IUser | null = null;
let initialSessionKey: string | null = null;
if (DEMO_MODE) {
  // tslint:disable:object-literal-sort-keys
  initialDemoSessions = [
    {
      name: "2017-2018",
      is_current: true,
      coordinator_name: "Prof. I.P. Freely",
      coordinator_email: "i.p.freely@not.dundee.ac.uk",
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
      name: "2016-2017",
      is_current: false,
      coordinator_name: "Prof. I.P. Freely",
      coordinator_email: "i.p.freely@not.dundee.ac.uk",
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

  initialSessionKey = "0xDEADBEEF";
  // tslint:enable:object-literal-sort-keys
}

const COMMIT_WORKING = {
  isWorking: true,
  type: Mutations.SET_IS_WORKING,
};

const COMMIT_NOT_WORKING = {
  isWorking: false,
  type: Mutations.SET_IS_WORKING,
};

// tslint:disable:object-literal-sort-keys
export default new Vuex.Store({
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
    session_key: initialSessionKey as string | null,
    user: initialUser as IUser | null,
    working: false,
  },
  mutations: {
    [Mutations.ADD_MARKED_PROJECT](state, payload) {
      if (state.user) {
        state.user.marked_projects = _.union([payload.project], state.user.marked_projects);
      }
    },
    [Mutations.RM_MARKED_PROJECT](state, payload) {
      if (state.user) {
        state.user.marked_projects = _.without(state.user.marked_projects, payload.project);
      }
    },
    [Mutations.SET_SELECTED_PROJECTS](state, payload) {
      if (state.user) {
        state.user.selected_projects = payload.projects;
      }
    },
    [Mutations.SET_SELECTION_COMMENT](state, payload) {
      if (state.user) {
        state.user.selection_comment = payload.comment;
      }
    },
    [Mutations.SET_USER_AND_SESSION](state, payload) {
      state.user = payload.user;
      state.session_key = payload.session_key;
    },
    [Mutations.NEW_PROJECT](state, payload) {
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
    [Mutations.EDIT_PROJECT](state, payload) {
      const session = _.first(state.available_sessions.filter((val) => val.is_current))!;
      // TODO: Send to server.
      const project = payload.project!;
      const idx = _.findIndex(session.projects, (ent) => ent.id === project.id);
      session.projects[idx] = project;
    },
    [Mutations.RM_PROJECT](state, payload) {
      const session = _.first(state.available_sessions.filter((val) => val.is_current))!;
      // TODO: Send to server.
      session.projects = _.filter(session.projects, (p: IProject) => {
        return p.id !== payload.project;
      });
    },
    [Mutations.ARCHIVE_SESSION](state, payload) {
      // TODO
    },
    [Mutations.PURGE_SESSION](state, payload) {
      // TODO
    },
    [Mutations.SET_IS_WORKING](state, payload) {
      state.working = payload.isWorking;
    },
  },
  actions: {
    [Actions.ADD_MARKED_PROJECT](ctx, payload) {
      // TODO
    },
    [Actions.RM_MARKED_PROJECT](ctx, payload) {
      // TODO
    },
    [Actions.SET_SELECTED_PROJECTS](ctx, payload) {
      // TODO
    },
    [Actions.SET_SELECTION_COMMENT](ctx, payload) {
      // TODO
    },
    async [Actions.NEW_PROJECT](ctx, payload) {
      ctx.commit(COMMIT_WORKING);
      // TODO: Server stuff. In the meantime, fake a delay.
      await new Promise((resolve) => _.delay(resolve, 2000));
      ctx.commit({
        type: Mutations.NEW_PROJECT,
        project: payload.project,
      });
      ctx.commit(COMMIT_NOT_WORKING);
    },
    [Actions.EDIT_PROJECT](ctx, payload) {
      // TODO
    },
    [Actions.RM_PROJECT](ctx, payload) {
      // TODO
    },
    [Actions.ARCHIVE_SESSION](ctx, payload) {
      // TODO
    },
    [Actions.PURGE_SESSION](ctx, payload) {
      // TODO
    },
  },
});
// tslint:enable:object-literal-sort-keys
