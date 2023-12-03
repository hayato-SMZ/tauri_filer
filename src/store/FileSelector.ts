import { defineStore } from "pinia";

export const useFileSelectorStore = defineStore("fileSelector", {
  state: () => ({
    cursorwindw: 0,
    cursor: 0,
    windowFileCount: 1,
  }),
  getters: {
    getWindw: (state) => state.cursorwindw,
    getCursor: (state) => state.cursor,
  },
  actions: {
    // cursorをアップ（上に移動）
    upCursor() {
      if (this.cursor > 1) {
        this.cursor--;
      }
    },
    // cursorをダウン（下に移動）
    downCursor() {
      console.log(this.cursor);
      if (this.cursor < this.windowFileCount - 1) {
        this.cursor++;
      }
    },
    //windowを変更
    changeWindow(windowId: number, fileCount: number) {
      this.cursorwindw = windowId;
      this.windowFileCount = fileCount;
      if (this.cursor > fileCount - 1) {
        this.cursor = fileCount - 1;
      }
    },
    setFileCounter(fileLists: number) {
      this.windowFileCount = fileLists;
    },
  },
});
