import { createApp } from "vue";
import App from "./App.vue";
import "./style.css";
import { router } from './router'
import { RecycleScroller } from 'vue-virtual-scroller'
import 'vue-virtual-scroller/dist/vue-virtual-scroller.css'


const app = createApp(App)
app.use(router)
app.component('RecycleScroller', RecycleScroller)
app.mount("#app");
