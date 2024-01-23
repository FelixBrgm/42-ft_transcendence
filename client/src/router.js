// src/router.js
import { createRouter, createWebHistory } from 'vue-router';
import HomePage from './components/pages/HomePage.vue';
import AboutUs from './components/pages/AboutUs.vue';
import RulesPage from './components/pages/RulesPage.vue';
import ProfilePage from './components/pages/ProfilePage.vue';
import DevTest from './components/elements/DevTest.vue';

const router = createRouter({
  history: createWebHistory(),
  routes: [
  { path: '/', component: HomePage, meta: { title: 'Transcendence' }},
  { path: '/about', component: AboutUs, meta: { title: 'About Us' } },
  { path: '/rules', component: RulesPage, meta: { title: 'Rules' } },
  { path: '/dev', component: DevTest, meta: { title: 'Developer testing', backgroundColor: 'white'} },
  { path: '/profile',component: ProfilePage, meta: { title: 'Profile',} },
],
});

router.beforeEach((to, from, next) => {
  document.title = to.meta.title || 'Default Title';
  
  if (to.meta.backgroundColor) {document.body.style.backgroundColor = to.meta.backgroundColor;} 
  else { document.body.style.backgroundColor = 'black';}
  next();
});

export default router;
