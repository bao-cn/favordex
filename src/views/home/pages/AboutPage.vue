<template>
  <div class="h-screen w-full p-8 bg-slate-50/50 dark:bg-slate-950 flex flex-col gap-6 overflow-hidden selection:bg-primary/10">
    
    <Card class="flex-none border-none shadow-2xl shadow-slate-200/50 dark:shadow-none bg-white/70 dark:bg-slate-900/70 backdrop-blur-2xl rounded-[2.5rem] overflow-hidden">
      <CardContent class="p-8 flex items-center justify-between">
        <div class="flex items-center gap-8">
          <div class="relative group">
            <div class="absolute inset-0 bg-primary/20 rounded-3xl rotate-6 group-hover:rotate-12 transition-transform duration-500"></div>
            <div class="relative w-20 h-20 bg-linear-to-br from-primary to-blue-600 rounded-3xl flex items-center justify-center text-white shadow-xl shadow-primary/30">
              <Bookmark class="w-10 h-10" />
            </div>
          </div>
          <div class="space-y-1">
            <div class="flex items-baseline gap-3">
              <h1 class="text-5xl font-[1000] tracking-tighter text-slate-900 dark:text-white leading-none">Favordex</h1>
              <span class="text-xs font-black text-primary opacity-60">{{ version }}</span>
            </div>
            <p class="text-[12px] font-black text-slate-400 flex items-center gap-3">
              AI Smart Bookmark Manager 
            </p>
            <div class="flex gap-2 pt-1">
              <Badge variant="outline" class="text-[9px] px-2 py-0 border-slate-200 dark:border-slate-800 font-bold opacity-70">Tauri</Badge>
              <Badge variant="outline" class="text-[9px] px-2 py-0 border-slate-200 dark:border-slate-800 font-bold opacity-70">Rust</Badge>
              <Badge variant="outline" class="text-[9px] px-2 py-0 border-slate-200 dark:border-slate-800 font-bold opacity-70">Vue.js 3</Badge>
            </div>
          </div>
        </div>
        
        <div class="flex gap-3">
          <Button variant="outline" class="rounded-2xl h-12 border-2 font-black px-6" @click="handleClick('https://github.com/bao-cn/favordex')">
            <Github class="w-4 h-4 mr-2" /> 参与贡献
          </Button>
          <Button class="rounded-2xl h-12 px-8 font-black shadow-lg shadow-primary/20" disabled>
            <RefreshCw class="w-4 h-4 mr-2" /> 检查更新
          </Button>
        </div>
      </CardContent>
    </Card>

    <div class="flex-none grid grid-cols-1 md:grid-cols-12 gap-6">
      <Card class="md:col-span-5 border-none shadow-xl bg-white/60 dark:bg-slate-900/60 backdrop-blur-xl rounded-[2.5rem] overflow-hidden">
        <CardContent class="p-6 flex items-center gap-6">
          <img src="https://avatars.githubusercontent.com/u/57002549" 
               class="w-20 h-20 rounded-2xl border-2 border-white dark:border-slate-700 shadow-lg object-cover flex-none" />
          <div class="flex-1 min-w-0">
            <p class="text-[10px] font-black text-primary uppercase tracking-widest mb-1">Project Maintainer</p>
            <h3 class="text-2xl font-[1000] text-slate-800 dark:text-slate-100 truncate">Gabriel Bao</h3>
            <div class="flex gap-3 mt-3">
              <Button size="icon" variant="secondary" class="h-8 w-8 rounded-lg" @click="handleClick('https://github.com/bao-cn')">
                <Github class="w-4 h-4" />
              </Button>
              <Button size="icon" variant="secondary" class="h-8 w-8 rounded-lg" @click="handleClick('mailto:chinabga@gmail.com')">
                <Mail class="w-4 h-4" />
              </Button>
            </div>
          </div>
        </CardContent>
      </Card>

      <Card class="md:col-span-7 border-none shadow-xl bg-slate-900 text-white rounded-[2.5rem] relative overflow-hidden">
        <Scale class="absolute -right-6 -bottom-6 w-32 h-32 opacity-10 -rotate-12" />
        <CardContent class="p-6 h-full flex flex-col justify-center">
          <div class="flex items-center gap-3 mb-2">
            <Badge class="bg-emerald-500/20 text-emerald-400 border border-emerald-500/30 px-2 py-0 text-[10px] font-black">MIT LICENSE</Badge>
            <h3 class="text-xl font-black italic tracking-tight">Open Source & Free</h3>
          </div>
          <p class="text-slate-400 text-xs leading-relaxed max-w-md font-medium">
            完全开源，您可以自由地使用、修改、分发本软件，甚至用于商业用途。
          </p>
          <Button variant="link" class="text-emerald-400 p-0 h-auto self-start font-black text-xs mt-3 hover:no-underline"
            @click="handleClick('https://github.com/bao-cn/favordex/blob/main/LICENSE')">
            查看协议全文 →
          </Button>
        </CardContent>
      </Card>
    </div>

    <Card class="flex-1 border-none shadow-xl bg-white/60 dark:bg-slate-900/60 backdrop-blur-xl rounded-[2.5rem] flex flex-col min-h-0">
      <CardHeader class="flex-none flex flex-row items-center justify-between px-10 pt-8 pb-4">
        <div class="flex items-center gap-3">
          <Users2 class="w-5 h-5 text-slate-400" />
          <CardTitle class="text-[10px] font-[1000] uppercase tracking-[0.2em] text-slate-500">Community Contributors</CardTitle>
        </div>
        <span class="text-[10px] font-black px-3 py-1 bg-slate-100 dark:bg-slate-800 rounded-full text-slate-500">
          {{ contributors.length }} Persons Joined
        </span>
      </CardHeader>
      
      <CardContent class="flex-1 px-10 pb-8 overflow-y-auto custom-scrollbar">
        <div class="flex flex-wrap gap-4">
          <div v-for="contributor in contributors" :key="contributor.login" class="group relative">
            <div class="w-12 h-12 rounded-xl bg-slate-100 dark:bg-slate-800 border-2 border-transparent group-hover:border-primary transition-all cursor-pointer overflow-hidden shadow-sm">
              <img :src="contributor.avatar_url" class="w-full h-full object-cover" />
            </div>
            <div class="absolute -top-10 left-1/2 -translate-x-1/2 px-2 py-1 bg-slate-800 text-white text-[10px] rounded opacity-0 group-hover:opacity-100 transition-opacity whitespace-nowrap z-20">
              @{{ contributor.login }}
            </div>
          </div>
          <button class="w-12 h-12 rounded-xl border-2 border-dashed border-slate-300 dark:border-slate-700 flex items-center justify-center text-slate-400 hover:border-primary hover:text-primary transition-all">
            <Plus class="w-5 h-5" />
          </button>
        </div>
      </CardContent>
    </Card>

    </div>
</template>

<script setup lang="ts">
import {
  Bookmark, Users2, Scale, Github, Mail,
  Plus, RefreshCw
} from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { open } from '@tauri-apps/plugin-shell'
import { ref, onMounted } from 'vue'
import { getGitHubContributors, GitHubContributor } from '@/utils/githubUtil'
import { getVersion } from '@tauri-apps/api/app';

const version = ref('');


const handleClick = (url: string) => {
  open(url)
}

const contributors = ref<GitHubContributor[]>([])

onMounted( async () => {
  (async () => {
    try {
      contributors.value = await getGitHubContributors({
        owner: 'bao-cn',
        repo: 'favordex',
        perPage: 30,
        page: 1
      });

      contributors.value.slice(0, 5).forEach((contributor: GitHubContributor) => {
        console.log(`${contributor.login}: ${contributor.id} 次贡献`);
      });
    } catch (error) {
      console.error('请求失败：', error);
    }
  })();
  version.value = await getVersion();
})


</script>

<style scoped>
.custom-scrollbar::-webkit-scrollbar {
  width: 4px;
}
.custom-scrollbar::-webkit-scrollbar-track {
  background: transparent;
}
.custom-scrollbar::-webkit-scrollbar-thumb {
  background: rgba(156, 163, 175, 0.2);
  border-radius: 10px;
}
</style>