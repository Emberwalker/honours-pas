import * as _ from "lodash";
import Vue from "vue";
import Vuex from "vuex";
import Mutations from "../lib/Mutations";
import {IProject, ISession, IUser, UserType} from "../lib/Types";

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
      coordinator_email: "i.p.freely@dundee.ac.uk",
      projects: [
        {
          name: "Sample Project #1",
          supervisor_name: "Prof. I.P. Freely",
          supervisor_email: "i.p.freely@dundee.ac.uk",
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
          supervisor_name: "Dr. Frankenstein",
          supervisor_email: "frankenstein@dundee.ac.uk",
          additional_staff: ["The Monster <monster@dundee.ac.uk>"],
          description_md: "Some kind of project description again...",
          id: 1235,
        },
      ],
    },
    {
      name: "2016-2017",
      is_current: false,
      coordinator_name: "Prof. I.P. Freely",
      coordinator_email: "i.p.freely@dundee.ac.uk",
      projects: [
        {
          name: "Old Sample Project #1",
          supervisor_name: "Prof. I.P. Freely",
          supervisor_email: "i.p.freely@dundee.ac.uk",
          additional_staff: [],
          description_md: "Some kind of older project description...",
          id: 1233,
        },
      ],
    },
  ];

  initialUser = {
    name: "Test User",
    email: "test.user@dundee.ac.uk",
    marked_projects: [1234],
    selected_projects: [],
    user_type: UserType.Staff,
  };

  initialSessionKey = "0xDEADBEEF";
  // tslint:enable:object-literal-sort-keys
}

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
          return getters.current_session(state);
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
  },
  // tslint:disable:object-literal-sort-keys
  mutations: {
    [Mutations.ADD_MARKED_PROJECT](state, payload) {
      if (state.user) {
        // TODO: Sync to server
        state.user.marked_projects = _.union([payload.project], state.user.marked_projects);
      }
    },
    [Mutations.RM_MARKED_PROJECT](state, payload) {
      if (state.user) {
        // TODO: Sync to server
        state.user.marked_projects = _.without(state.user.marked_projects, payload.project);
      }
    },
    [Mutations.SET_SELECTED_PROJECTS](state, payload) {
      if (state.user) {
        // TODO: Sync to server
        state.user.selected_projects = payload.projects;
      }
    },
    [Mutations.SET_USER_AND_SESSION](state, payload) {
      state.user = payload.user;
      state.session_key = payload.session_key;
    },
    [Mutations.NEW_PROJECT](state, payload) {
      const session = _.first(state.available_sessions.filter((val) => val.is_current))!;
      // TODO: Set this ID from server response.
      const project = payload.project;
      let maxId = 0;
      _.forEach(session.projects, (p: IProject) => { if (p.id! > maxId) { maxId = p.id!; } });
      project.id = maxId + 1;
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
      // TODO
    },
  },
  // tslint:enable:object-literal-sort-keys
});
