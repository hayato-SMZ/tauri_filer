<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Greet from "./components/Greet.vue";
import { useFileSelectorStore } from "./store/fileSelector";
const selecter = useFileSelectorStore();
const keydown = (e: any) => {
  if (e.key == "ArrowDown") {
    console.log("down");
    selecter.downCursor;
  } else if (e.key == "ArrowUp") {
    selecter.upCursor;
  } else if (e.key == "Enter") {
    console.log(selecter.cursor);// todo
  }
};
</script>

<template>
  <v-app @keydown="keydown">
    <v-main>
      <Greet />
    </v-main>
  </v-app>
</template>

<style scoped></style>
