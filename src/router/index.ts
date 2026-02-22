import { createRouter, createWebHashHistory } from 'vue-router'

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    {
      path: '/',
      component: () => import('../views/main/index.vue'),
    },
    {
      path: '/desktop',
      component: () => import('../views/desktop/index.vue'),
    },

  ],
})
export default router
