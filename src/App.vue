<script setup lang="ts">
import { onMounted } from "vue";
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import FilerWindow from "./components/FilerWindow.vue";
import { useFileSelectorStore } from "./store/fileSelector";
const selecter = useFileSelectorStore();
const keydown = (e: any) => {
  console.log(e.key);
  if (e.key == "ArrowDown") {
    selecter.downCursor();
    console.log(selecter.getCursor);
  } else if (e.key == "ArrowUp") {
    selecter.upCursor();
  } else if (e.key == "Enter") {
    console.log(selecter.cursor); // todo
  }
};

onMounted(() => {
  window.addEventListener("keydown", keydown);
});
</script>

<template>
  <v-app>
    <v-main >
      <v-container style="height: 70vh;overflow: hidden;">
        <v-row>
          <v-col>
            <FilerWindow :windowid="0"/>
          </v-col>
          <v-col>
            <FilerWindow :windowid="1"/>
          </v-col>
        </v-row>
      </v-container>
    </v-main>
    <v-footer app>
      <span class="text-center">&copy; 2021</span>
    </v-footer>
  </v-app>
</template>

<style scoped></style>
