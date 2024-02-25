import { createApp } from 'vue';
import App from './App.vue';
import router from './router';
import store from './store'; 
import axios from 'axios';
import 'bootstrap/dist/css/bootstrap.css'

axios.defaults.baseURL = ''; // Set your base URL here

const app = createApp(App);
app.use(store); // Use the Vuex store
app.use(router);
app.mount('#app');
