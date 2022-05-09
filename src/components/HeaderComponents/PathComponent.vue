<template>
  <div>
    <router-link :to="path" class="text-gray-400 text-xl mr-2">{{ id }}</router-link>
    <span class="text-gray-400 text-xl mx-2">/</span>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "PathComponent",
  props: {
    id: String,
    idArr: Array,
  },
  data() {
    return {
      path: "",
    };
  },
  mounted: function () {
    if (!this.id || !this.idArr) return;
    this.path = "/notes";
    if (this.id != "Notes") {
      let newArr = [] as string[];
      for (let i = 0; i < this.idArr.length; i++) {
        if (this.idArr[i] === this.id) {
          newArr.push((this.idArr as any)[i]);
          break;
        } else {
          newArr.push((this.idArr as any)[i]);
        }
      }
      this.path += "/" + JSON.stringify(newArr);
    }
    //   this.path = "/notes/" + JSON.stringify(this.idArr);
    console.log("Header Path:" + this.path);
  },
});
</script>
