<script setup lang="ts">
import { ref, onMounted,watch } from "vue";
import { invoke } from "@tauri-apps/api/tauri";


const dirlist = async () => {
  list.value = await invoke("rootdir", { path: path.value });
  console.log(list.value)
};
const list = ref<string[]>([])
const path = ref("")
const update = (e: any) => {
  path.value += "/" + e.target.innerText;
};
watch(path, dirlist);

onMounted(() => {
  dirlist();
});
</script>

<template>
  <div>
    <div>dir = <input v-model="path"> </div><button @click="dirlist">get</button>
    <div>
      <ul>
        <li v-for="item in list" :key="item" @click="update">{{ item }}</li>
      </ul>
    </div>
  </div>
</template>
