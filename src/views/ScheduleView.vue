<template>
  <div class="">
    <div class="flex justify-between">
      <div class="text-gray-400 text-xl m-4">Your Schedule</div>
      <div>
        <button class="mx-6 my-4 h-12 w-32 rounded bg-cyan-300">Add a class</button>
        <button class="mx-6 my-4 h-12 w-32 rounded bg-red-400">Remove a class</button>
      </div>
    </div>
    <CreateClassModalVue></CreateClassModalVue>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import CreateClassModalVue from "@/components/Modals/CreateClassModal.vue";

export default defineComponent({
  components: {
    CreateClassModalVue,
  },
  data() {
    return {
      showCreateModal: false,
    };
  },
  methods: {},
});
</script>
