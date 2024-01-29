import { createRouter, createWebHistory } from 'vue-router';
import store from './store';
import HomePage from './components/pages/HomePage.vue';
import AboutUs from './components/pages/AboutUs.vue';
import RulesPage from './components/pages/RulesPage.vue';
import ProfilePage from './components/pages/ProfilePage.vue';
import DevTest from './components/elements/DevTest.vue';
import PongGame from './components/elements/PongGame.vue';
import LoginPage from './components/pages/LoginPage';
import axios from 'axios';

const router = createRouter({
	history: createWebHistory(),
	routes: [
		{ path: '/', component: HomePage, meta: { title: 'Transcendence' } },
		{ path: '/login', component: LoginPage, meta: { title: 'Login' } },
		{ path: '/about', component: AboutUs, meta: { title: 'About Us' } },
		{ path: '/rules', component: RulesPage, meta: { title: 'Rules' } },
		{ path: '/dev', component: DevTest, meta: { title: 'Developer testing', backgroundColor: 'white' } },
		{ path: '/profile', component: ProfilePage, meta: { title: 'Profile', } },
		{ path: '/pong', component: PongGame, meta: { title: 'Pong', } },
	],
});

router.beforeEach(async (to, from, next) => {
	console.log(to);
	(from);
	document.title = to.meta.title || 'Default Title';

	if (to.meta.backgroundColor) {
		document.body.style.backgroundColor = to.meta.backgroundColor;
	} else { 
		document.body.style.backgroundColor = 'black';
	}

	if (to.path === '/logged_in' && !store.state.auth.user) {
		try {
			const response = await axios.get('http://127.0.0.1:8080/user', {
				withCredentials: true,
			});
			store.commit('auth/setUser', response.data);
			console.log(store.state.auth.user);
			next('/');
		} catch (error) {
			console.error('Error fetching user data:', error);
			next('/login');
		}
	}

	if (to.path === "/login" && store.state.auth.user != null) {
		next('/');
	}
	else if (to.meta.requiresAuth && store.state.auth.user == null) {
		next('/login');
	} else {
		next();
	}
});

export default router;