<template>
  <tr>
    <router-link :to="'/notes/' + JSON.stringify(pathArr)" class="text-center justify-center align-middle mt-5 text-gray-400 h-16">
      <div class="flex flex-row justify-between">
        <div>asd</div>
        <td class="w-32 text-left">{{ id }}</td>
        <td class="w-32">5</td>
        <td class="w-32 text-right">yesterday</td>
      </div>
    </router-link>
  </tr>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "FolderItem",
  props: {
    id: String,
    parent: String,
  },
  data() {
    return {
      pathArr: [] as string[],
    };
  },
  mounted: function () {
    if (!this.parent || !this.id) return;
    this.pathArr = JSON.parse(this.parent);
    this.pathArr.push(this.id);
    console.log("FolderItem: " + "/notes/" + JSON.stringify(this.pathArr));
  },
});
</script>
