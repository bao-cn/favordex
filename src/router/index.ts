import StorageUtil from '@/utils/storageUtil'
import { createRouter, createWebHashHistory } from 'vue-router'

export const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    {
      name: 'Home',
      path: '/',
      redirect: '/dashboard',
      component: () => import('@/views/home/HomeView.vue'),
      children: [
        {
          name: 'Dashboard',
          path: 'dashboard',
          component: () => import('@/views/home/pages/HomePage.vue')
        },
        {
          name: 'Category',
          path: 'category',
          component: () => import('@/views/home/pages/CategoryPage.vue')
        },
        {
          name: 'Settings',
          path: 'settings',
          component: () => import('@/views/home/pages/SettingsPage.vue')
        },
        {
          name: 'About',
          path: 'about',
          component: () => import('@/views/home/pages/AboutPage.vue')
        }
      ]
    },
    {
      name: 'OOBE',
      path: '/oobe',
      component: () => import('@/views/oobe/OobeView.vue')
    }
  ]
})

router.beforeEach((to, _, next) => {
  const isOobeCompleted = StorageUtil.get<boolean>('isInit') === true

  if (!isOobeCompleted && to.name !== 'OOBE') {
    next({ name: 'OOBE' })
  } else if (isOobeCompleted && to.name === 'OOBE') {
    next({ name: 'Dashboard' })
  } else {
    next()
  }
})