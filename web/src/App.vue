<template>
  <div id="app">
    <nav-header/>
    <div class="container h-100">
      <router-view/>
    </div>
    <div class="modal fade" ref="workingModal" id="workingModal" tabindex="-1" role="dialog" aria-labelledby="workingModalLabel"
         aria-hidden="true" data-backdrop="static" data-keyboard="false">
      <div class="modal-dialog" role="document">
        <div class="modal-content">
          <div class="modal-header bg-primary text-white">
            <h5 class="modal-title" id="archiveModalLabel">Working...</h5>
          </div>
          <div class="modal-body">
            <div class="container">
              <span id="app-working-spinner"><feather :spin="true" :large="true" icon="refresh-cw"/></span>
            </div>
          </div>
        </div>
      </div>
    </div>
    <div class="modal fade" ref="errorModal" id="errorModal" tabindex="-1" role="dialog" aria-labelledby="errorModalLabel"
         aria-hidden="true" data-backdrop="static" data-keyboard="false">
      <div class="modal-dialog" role="document">
        <div class="modal-content">
          <div class="modal-header bg-danger stripes text-white">
            <h5 class="modal-title" id="errorModalLabel">Error</h5>
          </div>
          <div class="modal-body">
            <div class="container">
              <p>We're sorry, an error has occurred: {{ errorText }}</p>
              <p>Please try refreshing the page.</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
  import $ from "jquery";
  import Vue from "vue";
  import Feather from "./components/Feather.vue";
  import NavHeader from "./components/NavHeader.vue";

  export default Vue.extend({
    components: {
      "feather": Feather,
      "nav-header": NavHeader,
    },
    computed: {
      errorText(): string {
        const err = this.$store.state.error;
        if (err) {
          return err.human;
        } else {
          // This shouldn't be possible, if there is an error.
          return "No descriptions available.";
        }
      },
      isErrored(): boolean {
        return this.$store.state.error !== null;
      },
      isWaiting(): boolean {
        return this.$store.state.working;
      },
    },
    name: "app",
    mounted() {
      // We add these to handle 'quick' changes to working state during transitions
      const working = $(this.$refs.workingModal) as any;
      working.on("shown.bs.modal", () => {
        if (!this.isWaiting) {
          working.modal("hide");
        }
      });
      working.on("hidden.bs.modal", () => {
        if (this.isWaiting) {
          working.modal("show");
        }
      });
    },
    watch: {
      isErrored(newVal: boolean) {
        const modal: any = $(this.$refs.errorModal);
        if (newVal) {
          modal.modal("show");
        } else {
          modal.modal("hide");
        }
      },
      isWaiting(newVal: boolean) {
        const modal: any = $(this.$refs.workingModal);
        if (newVal) {
          modal.modal("show");
        } else {
          modal.modal("hide");
        }
      },
    },
  });
</script>

<style lang="scss">
  @import "scss/custom";

  #app {
    margin-top: 4rem;
    height: calc(100vh - 4rem);
  }

  #app-working-spinner {
    position: relative;
    left: 44%;
  }

  .modal-body {
    display: block;
  }
</style>
