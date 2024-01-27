// src/router.js
import { createRouter, createWebHistory } from 'vue-router';
import HomePage from './components/pages/HomePage.vue';
import AboutUs from './components/pages/AboutUs.vue';
import RulesPage from './components/pages/RulesPage.vue';
import ProfilePage from './components/pages/ProfilePage.vue';
import DevTest from './components/elements/DevTest.vue';
import PongGame from './components/elements/PongGame.vue';
import LoginPage from './components/pages/LoginPage';

const router = createRouter({
	history: createWebHistory(),
	routes: [
		{ path: '/', component: HomePage, meta: { title: 'Transcendence' } },
		{ path: '/login', component: LoginPage , meta: { title: 'Login' } },
		{ path: '/about', component: AboutUs, meta: { title: 'About Us' } },
		{ path: '/rules', component: RulesPage, meta: { title: 'Rules' } },
		{ path: '/dev', component: DevTest, meta: { title: 'Developer testing', backgroundColor: 'white' } },
		{ path: '/profile', component: ProfilePage, meta: { title: 'Profile', } },
		{ path: '/pong', component: PongGame, meta: { title: 'Pong', } },
	],
});

router.beforeEach((to, from) => {
	(from);
	document.title = to.meta.title || 'Default Title';
	if (to.meta.backgroundColor) { document.body.style.backgroundColor = to.meta.backgroundColor; }
	else { document.body.style.backgroundColor = 'black'; }

	console.log(auth);
	console.log(to);
	if (to.path === '/logged_in') {
		auth.logged_in = true;
		return '/';
	}
	if (to.path !== "/login" && auth.logged_in == false) {
		return '/login';
	}
	if (to.path === "/login" && auth.logged_in) {
		return true;
	}


	return true;
});

export default router;


// Login storing
import { reactive } from 'vue'

export const auth = reactive({
	user: null,
	logged_in: false,
})