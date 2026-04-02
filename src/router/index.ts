import StorageUtil from '@/utils/storageUtil'
import { createRouter, createWebHashHistory } from 'vue-router'

export const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    {
      name: 'Home',
      path: '/',
      component: () => import('@/views/home/HomeView.vue')
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
    next({ name: 'Home' })
  } else {
    next()
  }
})