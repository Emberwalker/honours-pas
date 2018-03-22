<template>
<div v-if="show_form" class="container login-container h-100">
  <div class="h-100 row justify-content-center align-items-center">
    <div class="col-lg-6 col-md-8 col-sm-10 col-12">
      <div class="card d-block">
        <h3 class="card-header text-white bg-primary">
          <feather icon="shield"/>Login
        </h3>
        <div class="card-body">
          <div class="alert alert-danger" role="alert" :hidden="!show_err">
            Login failed. Check your username and password.
          </div>
          <form @submit.self.prevent="login()">
            <div class="form-group">
              <label for="username-field">Email Address</label>
              <input type="email" class="form-control form-control-sm" id="username-field" placeholder="Email Address" autocomplete="username" v-model="username">
            </div>
            <div class="form-group">
              <label for="password-field">Password</label>
              <input type="password" class="form-control form-control-sm" id="password-field" placeholder="Password" autocomplete="current-password" v-model="password">
            </div>
            <div class="container">
              <div class="row justify-content-end">
                <button type="submit" class="btn btn-sm btn-primary">Login</button>
              </div>
            </div>
          </form>
        </div>
      </div>
    </div>
  </div>
</div>
</template>

<script lang="ts">
import _ from "lodash";
import Vue from "vue";
import HTTP from "../lib/HTTP";
import Mutations from "../lib/Mutations";
import { IProject, IProjectSelection, ISession, IUser, UserType } from "../lib/Types";
import { COMMIT_NOT_WORKING, COMMIT_WORKING, getErrorCommit } from "../stores";

export default Vue.extend({
  computed: {
    server_opts(): any | null {
      return this.$store.state.server_opts;
    },
  },
  data() {
    return {
      password: "",
      show_err: false,
      show_form: false,
      username: "",
    };
  },
  methods: {
    update_opts() {
      if (this.server_opts) {
        // First check if we've already got a valid session.
        this.$store.commit(COMMIT_WORKING);
        HTTP.get("/whoami").then((res) => {
          this.onSuccess(res.data);
        }).catch((err) => {
          // Need to make a new session.
          if (this.server_opts.auth === "oauth2") {
            window.location.replace(this.server_opts.oauth2_url);
          } else {
            this.show_form = true;
          }
        }).finally(() => {
          this.$store.commit(COMMIT_NOT_WORKING);
        });
      } else {
        this.show_form = false;
      }
    },
    login() {
      this.$store.commit(COMMIT_WORKING);
      HTTP.post("/auth", {
        password: this.password,
        username: this.username,
      }).then((res) => {
        this.onSuccess(res.data);
      }).catch((err) => {
        console.error("Error pinging /auth", err);
        this.show_err = true;
      }).finally(() => {
        this.$store.commit(COMMIT_NOT_WORKING);
      });
    },
    onSuccess(whoami: any) {
      this.$store.commit(COMMIT_WORKING);
      let utype = UserType.Student;
      switch (whoami.user_type as string) {
        case "admin":
          utype = UserType.Administrator;
          break;
        case "staff":
          utype = UserType.Staff;
          break;
      }

      HTTP.get("/sessions/complete").then((sRes) => {
        const sessionsRaw: any[] = sRes.data.sessions;
        const projectsRaw: any[] = sRes.data.projects;

        const sessions = sessionsRaw.map((s) => {
          const projs = _.filter(projectsRaw, (it) => it.session === s.session.id) as IProject[];
          const session: ISession = {
            id: s.session.id as number,
            is_current: s.is_current as boolean,
            name: s.session.name as string,
            projects: projs,
            supervisor_email: s.session.supervisor_email as string,
            supervisor_name: s.session.supervisor_name as string,
          };
          return session;
        });

        this.$store.commit({
          sessions,
          type: Mutations.SET_PROJECTS_AND_SESSIONS,
        });

        if (utype === UserType.Student) {
          return HTTP.get("/me/marks").then((res) => {
            const user: IUser = {
              email: whoami.email as string,
              marked_projects: res.data.projects as number[],
              name: whoami.name as string,
              selected_projects: [] as IProjectSelection[],
              selection_comment: "",
              user_type: utype,
            };
            this.$store.commit({
              type: Mutations.SET_USER,
              user,
            });
            this.$router.replace("/");
          });
        } else {
          const user: IUser = {
            email: whoami.email as string,
            marked_projects: [] as number[],
            name: whoami.name as string,
            selected_projects: [] as IProjectSelection[],
            selection_comment: "",
            user_type: utype,
          };
          this.$store.commit({
            type: Mutations.SET_USER,
            user,
          });
          this.$router.replace("/");
        }
      }).catch((err) => {
        this.$store.commit(getErrorCommit("An error occurred fetching user/project information.", err));
      }).finally(() => {
        this.$store.commit(COMMIT_NOT_WORKING);
      });
    },
  },
  mounted() {
    if (this.$store.state.user) {
      this.$router.replace("/");
      return;
    }
    this.update_opts();
  },
  name: "Login",
  watch: {
    server_opts(newVal: any | null, oldVal: any | null) {
      if (!oldVal && newVal) {
        this.update_opts();
      }
    },
  },
});
</script>

<style scoped lang="scss">
.btn-right {
  float: right;
}
</style>
