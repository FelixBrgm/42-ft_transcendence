import { createApp } from 'vue'
import App from './App.vue'
import axios from 'axios';

createApp(App).mount('#app')

	axios.get('http://127:8000/api/hello').then((response) => {
		console.log(response.data);
	}).catch((error) => {
		console.error(error);
	});

