<template>
  <span class="feather-icon-svg" :class="nopad ? 'no-pad' : ''" v-html="getIcon()"></span>
</template>

<script lang="ts">
  import * as feather from "feather-icons";
  import Vue from "vue";

  export default Vue.extend({
    methods: {
      getIcon(): string {
        return feather.icons[this.icon].toSvg();
      },
    },
    name: "Feather",
    props: {
      icon: {
        required: true,
        type: String,
      },
      nopad: {
        default: false,
        required: false,
        type: Boolean,
      },
    },
  });
</script>

<style scoped lang="scss">

  .no-pad {
    padding-right: 0;
  }

</style>
