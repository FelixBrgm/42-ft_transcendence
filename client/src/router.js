import { createRouter, createWebHistory } from 'vue-router';
import HomePage from './components/pages/HomePage.vue';
import AboutUs from './components/pages/AboutUs.vue';
import RulesPage from './components/pages/RulesPage.vue';
import ProfilePage from './components/pages/ProfilePage.vue';
import DevTest from './components/elements/DevTest.vue';
import PongGame from './components/pages/GamePage.vue';
import LoginPage from './components/pages/LoginPage';
import NotFoundPage from './components/pages/NotFoundPage.vue'; // Import your custom 404 page

const router = createRouter({
    history: createWebHistory(),
    routes: [
        { path: '/', component: HomePage, meta: { title: 'Transcendence' } },
        { path: '/login', component: LoginPage, meta: { title: 'Login' } },
        { path: '/about', component: AboutUs, meta: { title: 'About Us' } },
        { path: '/rules', component: RulesPage, meta: { title: 'Rules' } },
        { path: '/dev', component: DevTest, meta: { title: 'Developer testing', backgroundColor: 'white' } },
        { path: '/profile', component: ProfilePage, meta: { title: 'Profile', requiresAuth: true } }, // Example of a route that requires authentication
        { path: '/pong', component: PongGame, meta: { title: 'Pong' } },
        { path: '/:pathMatch(.*)*', component: NotFoundPage }, // Wildcard route for not found pages
    ],
});

import store from './store';
import axios from 'axios';

router.beforeEach(async (to, from, next) => {

    document.body.style.backgroundColor = to.meta.backgroundColor || 'black';
	
    if (!store.state.auth.user && to.path != "/login") {
        try {
            const response = await axios.get('http://127.0.0.1:8080/user', { 
                withCredentials: true,
            });
            store.commit('auth/setUser', response.data);
        } catch (error) {
            console.error('Error fetching user data:', error);
            return next('/login');
        }
    }
    if (to.path === "/login" && store.state.auth.user != null) {
        return next('/');
    } 
    if (to.meta.requiresAuth && !store.state.auth.user) {
        return next('/login');
    }
    next();
});

export default router;
