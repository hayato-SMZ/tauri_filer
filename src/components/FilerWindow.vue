<script setup lang="ts">
import { ref, onMounted, watch, defineProps, defineExpose } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { computed } from "vue";
import { useFileSelectorStore } from "../store/FileSelector";

const props = defineProps({
  windowid: {
    type: Number,
    default: 0,
  },
});

const selectedfiles = ref<number[]>([]);

const cursorStore = useFileSelectorStore();

const cursorPosition = computed(() => {
  return cursorStore.getCursor;
});

const dirlist = () => {
  invoke("rootdir", { path: path.value != "" ? path.value : "/" })
    .then((res: unknown) => {
      const res_item = res as any[];
      list.value = res_item.filter((item: any) => item.is_dir) as any[];
      list.value = list.value.concat(
        res_item.filter((item: any) => !item.is_dir) as any[]
      );
      console.log(list.value);
      cursorStore.setFileCounter(props.windowid, list.value.length);
      selectedfiles.value = [];
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
const key_select = () => {
  if (selectedfiles.value.includes(cursorPosition.value)) {
    selectedfiles.value = selectedfiles.value.filter(
      (item: number) => item != cursorPosition.value
    );
  } else {
    selectedfiles.value.push(cursorPosition.value);
  }
};
const file_list = computed(() => {
  return list.value.filter((item: any) => !item.is_dir);
});
const path = ref("");
const update = () => {
  if (cursorPosition.value == 0) {
    cursorStore;
    goparent();
    return;
  }
  const item = list.value[cursorPosition.value - 1];
  if (item.is_dir) {
    path.value += "/" + item.file_name.replace(/\n|\/n/, "");
    cursorStore.resetCursor();
  } else {
    invoke("openfile", { path: path.value + "/" + item.file_name })
      .then((res: unknown) => {
        console.log(res);
      })
      .catch((err: string) => {
        console.log(err);
      });
  }
};
watch(path, dirlist);
watch(cursorPosition, () => {
  if (cursorStore.getWindow != props.windowid) return;
  setTimeout(() => {
    const element = document.getElementsByClassName("cursor")[0];
    if (element) {
      element.scrollIntoView({
        behavior: "smooth",
        block: "nearest",
        inline: "start",
      });
    }
  }, 10);
});

onMounted(() => {
  dirlist();
});
const goparent = () => {
  path.value = path.value.slice(0, path.value.lastIndexOf("/"));
  cursorStore.resetCursor();
};

const isCursor = (index: number) => {
  if (cursorStore.getWindow != props.windowid) {
    return "";
  }
  return cursorPosition.value == index ? "cursor" : "";
};

const isSelected = (index: number) => {
  return selectedfiles.value.includes(index) ? "selected" : "";
};

const getSelectList = () => {
  return selectedfiles.value.map(
    (item) => path.value + "/" + list.value[item - 1].file_name
  );
};

const getPath = () => {
  return path.value;
};

defineExpose({
  key_select,
  update,
  goparent,
  getSelectList,
  getPath,
});
</script>

<template>
  <div class="window">
    <div>di = <input v-model="path" /></div>
    <button @click="dirlist">get</button>
    <v-card>
      <v-list density="compact" lines="one">
        <v-list-item :class="isCursor(0)">
          <v-list-item-content>../</v-list-item-content>
        </v-list-item>
        <v-list-item
          v-for="(item, dirIndex) in dir_list"
          :key="item"
          :class="`${isSelected(dirIndex + 1)} ${isCursor(dirIndex + 1)} `"
          :prepend-icon="item.is_dir ? 'mdi-folder' : 'mdi-file'"
        >
          <v-list-item-content>{{ item.file_name }}</v-list-item-content>
        </v-list-item>
        <v-list-item
          v-for="(file, fileIndex) in file_list"
          :key="file"
          :class="`${isSelected(fileIndex + dir_list.length + 1)} ${isCursor(
            fileIndex + dir_list.length + 1
          )} `"
        >
          <v-list-item-content> {{ file.file_name }}</v-list-item-content>
        </v-list-item>
      </v-list>
    </v-card>
  </div>
</template>

<style lang="scss" scoped>
.selected {
  background-color: rgba(100, 255, 100, 0.3);
}
.cursor {
  background-color: #c0c0c0;
}
.window {
  height: 70vh;
  overflow: hidden;
}
</style>
