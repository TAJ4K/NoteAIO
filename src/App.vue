<template>
  <nav class="flex flex-col items-center w-40 h-screen overflow-hidden text-gray-400 bg-gray-900 rounded">
    <div class="flex items-center w-full px-3 mt-3">
      <div class="ml-2 text-sm font-bold">🔗NoteAIO</div>
    </div>
    <div class="w-full px-2">
      <div class="flex flex-col items-center w-full mt-3 border-t border-gray-700">
        <router-link class="flex items-center w-full h-12 px-3 mt-2 rounded hover:bg-gray-700 hover:text-gray-300" to="/">Home</router-link>
        <router-link class="flex items-center w-full h-12 px-3 mt-2 rounded hover:bg-gray-700 hover:text-gray-300" to="/notes">Notes</router-link>
        <router-link class="flex items-center w-full h-12 px-3 mt-2 rounded hover:bg-gray-700 hover:text-gray-300" to="/schedule">Schedule</router-link>
        <router-link class="flex items-center w-full h-12 px-3 mt-2 rounded hover:bg-gray-700 hover:text-gray-300" to="/settings">Settings</router-link>
      </div>
      <div class="flex flex-col items-center w-full mt-2 border-t border-gray-700"></div>
    </div>
  </nav>
  <router-view class="w-full" :key="$route.path" />
</template>

<script lang="ts">
import { defineComponent } from "vue";

// eslint-disable-next-line
const invoke = (window as any).__TAURI__.invoke;

export default defineComponent({
  name: "App",
  mounted: function () {
    invoke("create_parent_dir");
  },
});
</script>

<style>
@font-face {
  font-family: "mullish";
  src: url("./assets/Mulish-VariableFont_wght.ttf");
}

body {
  background-color: #0f1d2a;
  margin: 0px;
  height: 100vh;
}

#app {
  font-family: mullish;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  display: flex;
}

nav a.router-link-exact-active {
  background-color: #162b40;
  border-radius: 5px;
}
</style>
