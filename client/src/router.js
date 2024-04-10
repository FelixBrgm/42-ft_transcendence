import { createRouter, createWebHistory } from 'vue-router';
import HomePage from './components/pages/HomePage.vue';
import PeoplePeople from './components/pages/PeoplePeople.vue';
import RulesPage from './components/pages/RulesPage.vue';
import ProfilePage from './components/pages/ProfilePage.vue';
import PongGame from './components/pages/GamePage.vue';
import LocalGame from './components/pages/LocalGame.vue';
import AiGame from './components/pages/AiGame.vue';
import LoginPage from './components/pages/LoginPage';
import NamePage from './components/pages/NamePage';
import NotFoundPage from './components/pages/NotFoundPage.vue'; // Import your custom 404 page

const router = createRouter({
    history: createWebHistory(),
    routes: [
        { path: '/', component: HomePage, meta: { title: 'Transcendence' } },
        { path: '/login', component: LoginPage, meta: { title: 'Login' } },
        { path: '/name', component: NamePage, meta: { title: 'Login' } },
        { path: '/people', component: PeoplePeople, meta: { title: 'PeoplePeople' } },
        { path: '/rules', component: RulesPage, meta: { title: 'Rules' } },
        { path: '/profile', component: ProfilePage, meta: { title: 'Profile', requiresAuth: true } }, // Example of a route that requires authentication
        { path: '/pong', component: PongGame, meta: { title: 'Pong' } },
        { path: '/local', component: LocalGame, meta: { title: 'Pong' } },
        { path: '/ai', component: AiGame, meta: { title: 'Pong' } },
        { path: '/:pathMatch(.*)*', component: NotFoundPage }, // Wildcard route for not found pages
    ],
});

import store from './store';
import axios from 'axios';

router.beforeEach(async (to, from, next) => {

    document.body.style.backgroundColor = to.meta.backgroundColor || 'black';

    if (!store.state.auth.user && to.path != "/login") {
        try {
            const response = await axios.get('/api/user', {
                withCredentials: true,
            });
            store.commit('auth/setUser', response.data);
        } catch (error) {
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
