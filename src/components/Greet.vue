<script setup lang="ts">
import { ref, onMounted, watch } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { computed } from "vue";
import { useFileSelectorStore } from "../store/FileSelector";

const cursorStore = useFileSelectorStore();

const cursorPosition = computed(() => {
  console.log(cursorStore.getCursor);
  return cursorStore.getCursor;
});

const dirlist = () => {
  invoke("rootdir", { path: path.value != "" ? path.value : "/" })
    .then((res: unknown) => {
      const res_item = res as any[];
      list.value = res_item.filter((item: any) => item.is_dir) as any[];
      list.value = list.value.concat(
        res_item.filter((item: any) => !item.is_file) as any[]
      );
      cursorStore.setFileCounter(list.value.length);
      console.log(list.value);
    })
    .catch((err: string) => {
      path.value = path.value.slice(0, path.value.lastIndexOf("/"));
      console.log(err);
    });
};
const list = ref<any[]>([]);
const dir_list = computed(() => {
  return list.value.filter((item: any) => item.is_dir);
});
const file_list = computed(() => {
  return list.value.filter((item: any) => !item.is_dir);
});
const path = ref("");
const update = (e: any) => {
  path.value += "/" + e.target.innerText.replace(/\n|\/n/, "");
};
watch(path, dirlist);

onMounted(() => {
  dirlist();
});
const goparent = () => {
  path.value = path.value.slice(0, path.value.lastIndexOf("/"));
};

const isCursor = (index: number) => {
  return cursorPosition.value == index ? "cursor" : "";
};
</script>

<template>
  <div>
    <div>dir = <input v-model="path" /></div>
    <button @click="dirlist">get</button>
    <v-card>
      <v-list density="compact" lines="one">
        <v-list-item @click="goparent()" :class="isCursor(0)">
          <v-list-item-content>../</v-list-item-content>
        </v-list-item>
        <v-list-item
          v-for="(item, index) in dir_list"
          :key="item"
          :class="isCursor(index + 1)"
          @click="update"
          :prepend-icon="item.is_dir ? 'mdi-folder' : 'mdi-file'"
        >
          <v-list-item-content>{{ item.file_name }}</v-list-item-content>
        </v-list-item>
        <v-list-item v-for="item in file_list" :key="item">
          <v-list-item-content>{{ item.file_name }}</v-list-item-content>
        </v-list-item>
      </v-list>
    </v-card>
  </div>
</template>

<style lang="scss" scoped>
.cursor {
  background-color: #999;
}
</style>
