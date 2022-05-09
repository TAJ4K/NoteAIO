<template>
  <div class="">
    <div>asd</div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "CreateClassModal",
  methods: {
    close() {
      this.$emit("close");
    },
  },
});
</script>
