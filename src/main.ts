import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import { createVuetify } from "vuetify";
import { createPinia } from "pinia";
import * as components from "vuetify/components";
import * as directives from "vuetify/directives";
import "vuetify/styles";
import "@mdi/font/css/materialdesignicons.css";
const Vuetify = createVuetify({
  components,
  directives,
  icons: {
    defaultSet: "mdi",
  },
});

const pinia = createPinia();
createApp(App).use(pinia).use(Vuetify).mount("#app");
