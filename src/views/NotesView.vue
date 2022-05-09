<template>
  <div class="notes">
    <div class="flex justify-between">
      <div class="my-4">
        <router-link to="/notes" class="text-gray-400 text-xl mx-2">Notes</router-link>
        <span class="text-gray-400 text-xl mx-2">/</span>
      </div>
      <div>
        <button class="mx-6 my-4 h-12 w-32 rounded bg-cyan-300" @click="showModal">New Folder</button>
      </div>
    </div>
    <div id="noteSection" class="flex flex-row flex-wrap mx-8 gap-8">
      <FolderComponent v-for="folder in folders" :folderName="folder" :key="folder" @delete="deleteFolder($event)"></FolderComponent>
    </div>
    <CreateFolderModal v-show="isModalVisible" @close="closeModal($event)" />
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import FolderComponent from "@/components/FolderComponent.vue";
import CreateFolderModal from "@/components/Modals/CreateFolderModal.vue";

// eslint-disable-next-line
const invoke = (window as any).__TAURI__.invoke;

type closeEvent = {
  folderName: string;
  created: boolean;
};

export default defineComponent({
  name: "NotesView",
  components: {
    FolderComponent,
    CreateFolderModal,
  },
  data() {
    return {
      files: [] as string[],
      folders: [] as string[],
      isModalVisible: false,
    };
  },
  mounted: function () {
    //populate files and folders
    invoke("return_list_of_notes", { folderName: "" }).then((message: Array<string>) => {
      message.forEach((file: string) => {
        if (file.includes(".txt")) {
          this.files.push(file);
        } else {
          this.folders.push(file);
        }
      });
    });
  },
  methods: {
    showModal() {
      this.isModalVisible = true;
    },
    closeModal(event: closeEvent) {
      console.log(event);
      this.isModalVisible = false;
      if (event.folderName && event.created) {
        this.folders.push(event.folderName);
      }
    },
    deleteFolder(folderName: string) {
      this.folders = this.folders.filter((folder: string) => folder !== folderName);
    },
  },
});
</script>
