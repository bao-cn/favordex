<template>
  <div
    class="flex h-screen bg-[#f8fafc] dark:bg-[#020617] text-slate-900 dark:text-slate-100 antialiased selection:bg-primary/10 overflow-hidden font-sans">

    <aside
      class="w-72 flex flex-col bg-white/70 dark:bg-slate-900/70 backdrop-blur-2xl border-r border-slate-200/50 dark:border-slate-800/50 relative z-20">
      <div class="p-8 flex items-center gap-3">
        <div
          class="w-10 h-10 rounded-2xl bg-linear-to-tr from-primary to-blue-500 flex items-center justify-center text-white shadow-lg shadow-primary/30">
          <Bookmark class="w-5 h-5" />
        </div>
        <div>
          <span class="font-black tracking-tighter text-xl block leading-tight">Favordex</span>
          <span class="text-[10px] font-bold text-primary/60 uppercase tracking-widest">AI 浏览器书签整理</span>
        </div>
      </div>

      <nav class="flex-1 px-4 space-y-1.5 mt-2">
        <button @click="activeTab = 'home'" :class="[
          'w-full flex items-center gap-3 px-5 py-3.5 rounded-2xl transition-all duration-300 font-bold text-sm group',
          activeTab === 'home'
            ? 'bg-primary text-white shadow-xl shadow-primary/20 scale-[1.02]'
            : 'text-slate-500 hover:bg-slate-100 dark:hover:bg-slate-800/50 hover:text-slate-800 dark:hover:text-slate-200'
        ]">
          <component :is="Home"
            :class="['w-5 h-5 transition-transform duration-500', activeTab === 'home' ? 'rotate-[-5deg] scale-110' : 'group-hover:scale-110']" />
          首 页
        </button>
        <button @click="activeTab = 'categories'" :class="[
          'w-full flex items-center gap-3 px-5 py-3.5 rounded-2xl transition-all duration-300 font-bold text-sm group',
          activeTab === 'categories'
            ? 'bg-primary text-white shadow-xl shadow-primary/20 scale-[1.02]'
            : 'text-slate-500 hover:bg-slate-100 dark:hover:bg-slate-800/50 hover:text-slate-800 dark:hover:text-slate-200'
        ]">
          <component :is="FolderTree"
            :class="['w-5 h-5 transition-transform duration-500', activeTab === 'categories' ? 'rotate-[-5deg] scale-110' : 'group-hover:scale-110']" />
          分 类
        </button>
      </nav>

      <div class="p-6 mt-auto gap-4">
        <button @click="activeTab = 'settings'" :class="[
          'w-full flex items-center gap-3 px-5 py-3.5 rounded-2xl transition-all duration-300 font-bold text-sm group',
          activeTab === 'settings'
            ? 'bg-primary text-white shadow-xl shadow-primary/20 scale-[1.02]'
            : 'text-slate-500 hover:bg-slate-100 dark:hover:bg-slate-800/50 hover:text-slate-800 dark:hover:text-slate-200'
        ]">
          <component :is="Settings2"
            :class="['w-5 h-5 transition-transform duration-500', activeTab === 'settings' ? 'rotate-[-5deg] scale-110' : 'group-hover:scale-110']" />
          设 置
        </button>
        <button @click="activeTab = 'about'" :class="[
          'w-full flex items-center gap-3 px-5 py-3.5 rounded-2xl transition-all duration-300 font-bold text-sm group mt-2',
          activeTab === 'about'
            ? 'bg-primary text-white shadow-xl shadow-primary/20 scale-[1.02]'
            : 'text-slate-500 hover:bg-slate-100 dark:hover:bg-slate-800/50 hover:text-slate-800 dark:hover:text-slate-200'
        ]">
          <component :is="Info"
            :class="['w-5 h-5 transition-transform duration-500', activeTab === 'about' ? 'rotate-[-5deg] scale-110' : 'group-hover:scale-110']" />
          关 于
        </button>
      </div>
    </aside>

    <main class="flex-1 relative overflow-y-auto scroll-smooth custom-scrollbar">
      <transition name="page-fade" mode="out-in">
        <div :key="activeTab" class="min-h-full p-10 max-w-7xl mx-auto">
          <HomePage v-if="activeTab === 'home'" />
          <CategoryPage v-if="activeTab === 'categories'" />
          <SettingsPage v-if="activeTab === 'settings'" />
          <AboutPage v-if="activeTab === 'about'" />
        </div>
      </transition>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import {
  Home, FolderTree, Settings2, Info, Bookmark,
} from 'lucide-vue-next'
import HomePage from './pages/HomePage.vue'
import CategoryPage from './pages/CategoryPage.vue'
import AboutPage from './pages/AboutPage.vue'
import SettingsPage from './pages/SettingsPage.vue'


const activeTab = ref('home')
</script>

<style>
.page-fade-enter-active,
.page-fade-leave-active {
  transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);
}

.page-fade-enter-from {
  opacity: 0;
  transform: translateY(20px) scale(0.98);
}

.page-fade-leave-to {
  opacity: 0;
  transform: translateY(-20px) scale(1.02);
}

:deep(.custom-el-tree) {
  background: transparent !important;
  color: inherit;
}

:deep(.el-tree-node__content) {
  height: 44px !important;
  border-radius: 14px;
  margin-bottom: 4px;
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
}

:deep(.el-tree-node__content:hover) {
  background-color: rgba(var(--primary-rgb), 0.08) !important;
}

:deep(.el-tree-node.is-current > .el-tree-node__content) {
  background-color: rgba(var(--primary-rgb), 0.15) !important;
  color: var(--primary);
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