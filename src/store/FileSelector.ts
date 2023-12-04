import { defineStore } from "pinia";

export const useFileSelectorStore = defineStore("fileSelector", {
  state: () => ({
    cursorwindow: 0,
    cursor: 0,
    windowFileCount: [] as number[],
  }),
  getters: {
    getWindow: (state) => state.cursorwindow,
    getCursor: (state) => state.cursor,
  },
  actions: {
    // cursorをアップ（上に移動）
    upCursor() {
      if (this.cursor > 0) {
        this.cursor--;
      }
    },
    // cursorをダウン（下に移動）
    downCursor() {
      console.log(this.cursor);
      const cursolerFilerFiles = this.windowFileCount[this.cursorwindow];
      if (this.cursor < cursolerFilerFiles) {
        this.cursor++;
      }
    },
    //windowを変更
    changeWindow(windowId: number) {
      this.cursorwindow = windowId;
      if (this.cursor > this.windowFileCount[this.cursorwindow] - 1) {
        this.cursor = this.windowFileCount[this.cursorwindow] - 1;
      }
    },
    setFileCounter(windowId: number, fileLists: number) {
      this.windowFileCount[windowId] = fileLists;
    },
  },
});
