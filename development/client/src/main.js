import { createApp } from 'vue'
import App from './App.vue'

createApp(App).mount('#app')

let ws = new WebSocket("ws://127.0.0.1:4242");

(ws)

