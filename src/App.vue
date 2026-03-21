<template>
  <div>
    <Toaster />
    <RouterView />
  </div>
</template>

<script setup lang="ts">
import 'vue-sonner/style.css'
import { Toaster } from '@/components/ui/sonner'
</script>

<style scoped>

</style>