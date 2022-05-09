<template>
  <div ref="modal" class="fixed inset-0 bg-gray-700/75 flex justify-center items-center" @click="handleBackdropClick" @keydown.esc="close()">
    <div class="bg-gray-900 overflow-x-auto flex flex-col rounded">
      <div class="p-4 flex relative text-gray-400 justify-between">
        <button class="absolute top-0 right-0 text-xl p-2 cursor-pointer font-bold text-gray-500 bg-transparent" @click="close()">&times;</button>
      </div>
      <div class="relative py-5 px-3">
        <div class="border rounded flex flex-col pr-2 pb-2 pt-1 pl-1">
          <span class="modal-title text-gray-300">Create New Folder</span>
          <input type="text" v-model="folderName" placeholder="Folder Name" class="pl-1 pt-1 outline-none bg-inherit w-42 text-white" @keydown.enter="createFolder" />
        </div>
      </div>
      <div class="p-4 flex flex-col justify-end">
        <button class="text-white bg-gray-500 rounded-sm" @click="createFolder">Create</button>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

// eslint-disable-next-line
const invoke = (window as any).__TAURI__.invoke;

export default defineComponent({
  name: "CreateFolderModal",
  data() {
    return {
      folderName: "",
    };
  },
  methods: {
    close(created = false) {
      this.$emit("close", { folderName: this.folderName, created: created });
    },
    createFolder() {
      invoke("create_folder", { folderName: this.folderName });
      this.close(true);
    },
    handleBackdropClick(event: Event) {
      if (event.target === event.currentTarget) {
        this.close();
      }
    },
  },
});
</script>
