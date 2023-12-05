<script setup lang="ts">
import { onMounted, ref } from "vue";
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import FilerWindow from "./components/FilerWindow.vue";
import { useFileSelectorStore } from "./store/fileSelector";
const selecter = useFileSelectorStore();
const keydown = (e: any) => {
  console.log(e.key);
  if (e.key == "ArrowDown") {
    selecter.downCursor();
  } else if (e.key == "ArrowUp") {
    selecter.upCursor();
  } else if (e.key == "ArrowRight") {
    selecter.changeWindow(1);
  } else if (e.key == "ArrowLeft") {
    selecter.changeWindow(0);
  } else if (e.key == "Backspace") {
    if (selecter.getWindow == 0) {
      console.log("window0ref");
      window0Ref.value?.goparent();
    } else {
      window1Ref.value?.goparent();
    }
  } else if (e.key == "Enter") {
    if (selecter.getWindow == 0) {
      console.log("window0ref");
      window0Ref.value?.update();
    } else {
      window1Ref.value?.update();
    }
  } else if (e.key == " ") {
    if (selecter.getCursor < 1) {
      return;
    }
    if (selecter.getWindow == 0) {
      console.log("window0ref");
      window0Ref.value?.key_select();
    } else {
      window1Ref.value?.key_select();
    }
  }
};

onMounted(() => {
  window.addEventListener("keydown", keydown);
});
const window0Ref = ref<InstanceType<typeof FilerWindow> | null>(null);
const window1Ref = ref<InstanceType<typeof FilerWindow> | null>(null);
</script>

<template>
  <v-app>
    <v-main>
      <v-container>
        <v-row>
          <v-col>
            <FilerWindow :windowid="0" ref="window0Ref" />
          </v-col>
          <v-col>
            <FilerWindow :windowid="1" ref="window1Ref" />
          </v-col>
        </v-row>
      </v-container>
    </v-main>
    <v-footer app>
      <span class="text-ceneeter">&copy; 2021</span>
    </v-footer>
  </v-app>
</template>

<style scoped></style>
