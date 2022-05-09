<template>
  <div class="flex flex-col justify-center">
    <span class="flex flex-row text-center justify-center text-gray-400 text-4xl pb-2" @mouseover="showEdit = true" @mouseleave="(showEdit = false), (edit = false), setName()">
      &nbsp;Welcome,&nbsp;
      <div class="flex flex-row">
        <span v-if="!edit">{{ user }}</span>
        <input v-if="edit" type="text" v-model="user" class="appearance-none bg-inherit w-40" />
        <div v-if="!showEdit" class="w-4"></div>
        <img v-if="showEdit" src="@/assets/pencil.svg" class="w-4 h-4 cursor-pointer" @click="edit = !edit" />
      </div>
    </span>
    <span class="text-center text-gray-400">The time is {{ time }}</span>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

// eslint-disable-next-line
const invoke = (window as any).__TAURI__.invoke;

export default defineComponent({
  data() {
    return {
      user: "",
      time: "",
      edit: false,
      showEdit: false,
    };
  },
  mounted: function () {
    this.setTime();
    this.getName();
    setInterval(() => this.setTime(), 1000);
  },
  methods: {
    setTime() {
      this.time = new Date().toLocaleTimeString("en-us", { weekday: "long" });
    },
    getName() {
      invoke("get_json").then((message: string) => {
        this.user = JSON.parse(message)?.name ?? "";
      });
    },
    setName() {
      console.log(this.user);
      invoke("write_name_to_json", { name: this.user });
    },
  },
});
</script>
