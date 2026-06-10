import { createApp } from "vue";
import App from "./App.vue";
// import { invoke } from "@tauri-apps/api/core";

createApp(App).mount("#app");

// 确保在 DOM 挂载且一帧渲染结束后，再通知 Rust 我们准备好了
// requestAnimationFrame(() => {
//     setTimeout(() => {
//         invoke('frontend_ready').catch(e => console.error("通知 Rust 失败:", e));
//     }, 100);
// });
