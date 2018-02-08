<template>
  <div id="app">
    <nav-header/>
    <div class="container">
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
    <!--<div id="app-working-overlay" v-if="isWaiting">
      <span id="app-working-spinner"><feather spin="true" large="true" icon="refresh-cw"/></span>
    </div> -->
  </div>
</template>

<script lang="ts">
  import Vue from "vue";
  import Feather from "./components/Feather.vue";
  import NavHeader from "./components/NavHeader.vue";

  export default Vue.extend({
    components: {
      "feather": Feather,
      "nav-header": NavHeader,
    },
    computed: {
      isWaiting(): boolean {
        return this.$store.state.working;
      },
    },
    name: "app",
    watch: {
      isWaiting(newVal: boolean) {
        // console.log("isWaiting update:", newVal);
        const modal: any = $(this.$refs.workingModal);
        // We do this song and dance in case the modal is still transitioning.
        if (newVal) {
          /*const onHidden = () => {
            modal.off("hidden.bs.modal", onHidden);
            modal.modal("show");
          };
          const onShown = () => {
            modal.off("hidden.bs.modal", onHidden);
            modal.off("shown.bs.modal", onShown);
          };
          modal.on("hidden.bs.modal", onHidden);*/
          modal.modal("show");
        } else {
          /*const onShown = () => {
            modal.off("shown.bs.modal", onShown);
            modal.modal("hide");
          };
          const onHidden = () => {
            modal.off("shown.bs.modal", onShown);
            modal.off("hidden.bs.modal", onHidden);
          };
          modal.on("shown.bs.modal", onShown);*/
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
  }

  #app-working-overlay {
    background: white;
    opacity: 0.5;
    position: fixed;
    width: 100%;
    height: 100%;
    padding: 0;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
  }

  #app-working-spinner {
    position: relative;
    left: 42%;
  }

  .modal-body {
    display: block;
  }
</style>
