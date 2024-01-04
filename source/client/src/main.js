
import { createApp } from 'vue'
import Vuex from 'vuex';
import App from './App.vue'
import VueNativeSock from 'vue-native-websocket';

createApp(App).mount('#app')

export default new Vuex.Store({
	state: {
		accessToken: null,
	},
	mutations: {
		setAccessToken(state, token) {
			console.log('the accessToken is set now----------');
			state.accessToken = token;
		}
	}
})

Vuex.use(VueNativeSock, 'http://127.0.0.1:8080/ws', {
  format: 'json',
  reconnection: true, // Enable automatic reconnection
  reconnectionAttempts: 5, // Number of attempts
  reconnectionDelay: 3000, // Delay between attempts in milliseconds
});

new Vuex({
  render: (h) => h(App),
}).$mount('#app');
