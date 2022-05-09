<template>
  <div class="w-32 h-32 bg-gray-800 rounded border border-blue-200 drop-shadow-xl" @click="navToFolder()">
    <div class="flex flex-col gap-2">
      <router-link :to="'/notes/' + JSON.stringify(['Notes', folderName])" class="text-center justify-center align-middle mt-5 text-gray-400 h-16">{{ folderName }}</router-link>
      <div class="flex flex-row justify-between top-max">
        <div class="mx-3 flex flex-row">
          <img src="@/assets/fileicon.svg" class="h-5 w-5 select-none" draggable="false" />
          <div id="fileNo" class="select-none">{{ count }}</div>
        </div>
        <button v-on:click="handleDelete" class="mx-3 z-10">
          <img id="deleteButton" :src="require(`@/assets/${iconName}`)" class="h-5 w-5 select-none" draggable="false" />
        </button>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

// eslint-disable-next-line
const invoke = (window as any).__TAURI__.invoke;

export default defineComponent({
  name: "FolderComponent",
  data() {
    return {
      deleteCount: 0,
      iconName: "trashcan.svg",
      count: 0,
      shouldRender: true,
    };
  },
  props: {
    folderName: String,
  },
  mounted: function () {
    console.log("/notes/" + JSON.stringify(["notes", this.folderName]));
    this.updateCount();
  },
  methods: {
    handleDelete() {
      this.deleteCount++;
      console.log(this.deleteCount);
      if (this.deleteCount === 1) {
        this.iconName = "check.svg";
      }
      if (this.deleteCount === 2) {
        invoke("delete_folder", { folderName: this.folderName });
        this.$emit("delete", this.folderName);
      }
    },
    updateCount() {
      invoke("return_list_of_notes", { folderName: this.folderName }).then((message: Array<string>) => {
        this.count = message.length;
      });
    },
    navToFolder() {
      console.log("Nav To Folder: " + this.folderName);
    },
  },
});
</script>
