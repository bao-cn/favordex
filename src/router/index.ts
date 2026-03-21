import { createRouter, createWebHashHistory } from 'vue-router'

export const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    {
      path: '/oobe',
      component: () => import('@/views/oobe/OobeView.vue')
    },
    {
      path: '/',
      component: () => import('@/views/home/HomeView.vue')
    }
  ]
})
