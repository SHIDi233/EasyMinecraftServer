import { createRouter, createWebHistory } from 'vue-router'
import Login from '../views/Login.vue'
import Base from '../views/Base.vue'
import Home from '../components/Home.vue'
import User from '../components/User.vue'
import Notice from '../components/Notice.vue'
import Safe from '../components/Safe.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/login',
      name: '/login',
      component: Login
    },
    {
      path: '/',
      name:'/',
      component: Base,
      children:[
        {
          path: '/home',
          name: '/home',
          component: Home
        },
        {
          path: '/users',
          name: '/users',
          component: User
        },
        {
          path: '/notice',
          name: '/notice',
          component: Notice
        },
        {
          path: '/safe',
          name: '/safe',
          component: Safe
        },
      ]
    },
  ]
})

router.beforeEach((to, from, next) => {
  if (to.path === '/login') return next();
  const tokenStr = window.localStorage.getItem('easy_mc_token');
  if (!tokenStr) return next('/login');
  next();
})

export default router