import { createApp } from 'vue'
import App from './App.vue'
import axios from 'axios';

createApp(App).mount('#app')

	axios.get('http://127.0.0.1:8081/hey').then((response) => {
		console.log(response.data);
	}).catch((error) => {
		console.error(error);
	});

