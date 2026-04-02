<template>
  <div>
    <Toaster />
    <RouterView />
  </div>
</template>

<script setup lang="ts">
import 'vue-sonner/style.css'
import { Toaster } from '@/components/ui/sonner'
import { useRouter } from 'vue-router'
import { onMounted } from 'vue'
import StorageUtil from '@/utils/storageUtil'

const router = useRouter()

onMounted(async () => {
  if (StorageUtil.get('isInit') === 'true') {
    router.push('/')
  } else {
    router.push('/oobe')
  }
})
</script>

<style scoped>

</style>