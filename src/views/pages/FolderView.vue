<template>
  <div>
    <div id="header">
      <div class="flex flex-row">
        <PathComponent v-for="id in idArr" class="my-4 mx-2" :key="id" :id="id" :idArr="idArr" />
      </div>
    </div>
    <div id="body">
      <div class="flex flex-row">
        <div class="w-3/5 bg-gray-800 mx-2">
          <table class="w-full">
            <tbody class="ml-2">
              <FolderItem v-for="folder in folders" :key="folder" :id="folder" :parent="id" />
              <NoteItem v-for="file in files" :key="file" :id="file" />
            </tbody>
          </table>
        </div>
        <div class="w-2/5"></div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import FolderItem from "@/components/ItemComponents/FolderItem.vue";
import NoteItem from "@/components/ItemComponents/NoteItem.vue";
import PathComponent from "@/components/HeaderComponents/PathComponent.vue";

// eslint-disable-next-line
const invoke = (window as any).__TAURI__.invoke;

export default defineComponent({
  name: "FolderView",
  props: {
    id: String,
  },
  data() {
    return {
      files: [] as string[],
      folders: [] as string[],
      currentId: this.id ? JSON.parse(this.id)[JSON.parse(this.id).length - 1] : "",
      idArr: this.id ? JSON.parse(this.id) : [],
    };
  },
  components: {
    FolderItem,
    NoteItem,
    PathComponent,
  },
  mounted: function () {
    console.log("ID:" + this.id);
    console.log("Current ID:" + this.currentId);
    let pathFolder = (JSON.parse(this.id as any) as string[]).filter((value: string, index: number) => {
      return index != 0;
    });
    console.log("Path:" + pathFolder);
    invoke("return_list_of_notes_from_array_path", { folderPath: pathFolder }).then((message: Array<string>) => {
      this.files = message.filter((file: string) => file.includes(".txt"));
      this.folders = message.filter((file: string) => !file.includes(".txt"));
    });
  },
});
</script>
