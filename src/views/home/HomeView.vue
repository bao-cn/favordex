<template>
  <div class="flex h-screen bg-[#f8fafc] dark:bg-[#020617] text-slate-900 dark:text-slate-100 antialiased selection:bg-primary/10 overflow-hidden font-sans">
    
    <aside class="w-72 flex flex-col bg-white/70 dark:bg-slate-900/70 backdrop-blur-2xl border-r border-slate-200/50 dark:border-slate-800/50 relative z-20">
      <div class="p-8 flex items-center gap-3">
        <div class="w-10 h-10 rounded-2xl bg-linear-to-tr from-primary to-blue-500 flex items-center justify-center text-white shadow-lg shadow-primary/30">
          <Bookmark class="w-5 h-5" />
        </div>
        <div>
          <span class="font-black tracking-tighter text-xl leading-tight flex items-center gap-1">Favordex</span>
          <span class="text-[10px] font-bold text-primary/60 uppercase tracking-widest">AI 赋能的书签整理工具</span>
        </div>
      </div>

      <nav class="flex-1 px-4 space-y-1.5 mt-2">
        <button v-for="item in navItems" :key="item.routeName" @click="router.push({ name: item.routeName })" :class="[
          'w-full flex items-center gap-3 px-5 py-3.5 rounded-2xl transition-all duration-300 font-bold text-sm group',
          route.name === item.routeName
            ? 'bg-primary text-white shadow-xl shadow-primary/20 scale-[1.02]'
            : 'text-slate-500 hover:bg-slate-100 dark:hover:bg-slate-800/50 hover:text-slate-800 dark:hover:text-slate-200'
        ]">
          <component :is="item.icon"
            :class="['w-5 h-5 transition-transform duration-500', route.name === item.routeName ? 'rotate-[-5deg] scale-110' : 'group-hover:scale-110']" />
          {{ item.label }}
        </button>
      </nav>

      <div class="p-6 mt-auto flex flex-col gap-2 border-t border-slate-100/50 dark:border-slate-800/50">
        <button v-for="item in bottomNavItems" :key="item.routeName" @click="router.push({ name: item.routeName })" :class="[
          'w-full flex items-center gap-3 px-5 py-3.5 rounded-2xl transition-all duration-300 font-bold text-sm group',
          route.name === item.routeName
            ? 'bg-primary text-white shadow-xl shadow-primary/20 scale-[1.02]'
            : 'text-slate-500 hover:bg-slate-100 dark:hover:bg-slate-800/50 hover:text-slate-800 dark:hover:text-slate-200'
        ]">
          <component :is="item.icon"
            :class="['w-5 h-5 transition-transform duration-500', route.name === item.routeName ? 'rotate-[-5deg] scale-110' : 'group-hover:scale-110']" />
          {{ item.label }}
        </button>
      </div>
    </aside>

    <main class="flex-1 relative overflow-hidden bg-background"> <RouterView v-slot="{ Component }">
    <transition name="page-fade" mode="out-in">
      <div :key="route.path" class="h-full w-full">
        <component :is="Component" />
      </div>
    </transition>
  </RouterView>
</main>
  </div>
</template>

<script setup lang="ts">
import { useRoute, useRouter } from 'vue-router'
import { Home, FolderTree, Settings2, Info, Bookmark } from 'lucide-vue-next'

const route = useRoute()
const router = useRouter()

const navItems = [
  { label: '首 页', routeName: 'Dashboard', icon: Home },
  { label: '分 类', routeName: 'Category', icon: FolderTree },
]

const bottomNavItems = [
  { label: '设 置', routeName: 'Settings', icon: Settings2 },
  { label: '关 于', routeName: 'About', icon: Info },
]
</script>

<style>
.page-fade-enter-active,
.page-fade-leave-active {
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.page-fade-enter-from {
  opacity: 0;
  transform: translateY(12px) scale(0.99);
}

.page-fade-leave-to {
  opacity: 0;
  transform: translateY(-12px) scale(1.01);
}

.custom-scrollbar::-webkit-scrollbar {
  width: 6px;
}
.custom-scrollbar::-webkit-scrollbar-track {
  background: transparent;
}
.custom-scrollbar::-webkit-scrollbar-thumb {
  background: #e2e8f0;
  border-radius: 10px;
}
.dark .custom-scrollbar::-webkit-scrollbar-thumb {
  background: #1e293b;
}
</style>