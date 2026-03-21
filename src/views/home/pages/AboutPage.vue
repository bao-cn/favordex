<template>
  <div class="space-y-10 animate-in fade-in slide-in-from-bottom-8 duration-700 pb-20">
    
    <section class="flex flex-col items-center justify-center pt-10 pb-4 text-center">
      <div class="relative w-24 h-24 mb-6 group">
        <div class="absolute inset-0 bg-primary/20 rounded-[2rem] rotate-12 group-hover:rotate-45 transition-transform duration-700"></div>
        <div class="relative w-full h-full bg-linear-to-tr from-primary to-blue-600 rounded-[2rem] flex items-center justify-center text-white shadow-2xl shadow-primary/30">
          <Bookmark class="w-10 h-10" />
        </div>
      </div>
      <h1 class="text-4xl font-black tracking-tighter text-slate-800 dark:text-slate-100">Favordex</h1>
      <p class="text-[10px] font-black text-slate-400 uppercase tracking-[0.3em] mt-2">Open Source AI Manager</p>
    </section>

    <div class="grid grid-cols-1 md:grid-cols-12 gap-6">
      
      <Card class="md:col-span-5 border-none shadow-xl shadow-slate-200/50 dark:shadow-none bg-white/60 dark:bg-slate-900/60 backdrop-blur-xl rounded-[2.5rem] overflow-hidden">
        <CardHeader class="pb-2">
          <CardTitle class="text-xs font-black uppercase tracking-widest text-primary flex items-center gap-2">
            <User class="w-3.5 h-3.5" /> Project Maintainer
          </CardTitle>
        </CardHeader>
        <CardContent class="flex items-center gap-5 p-8 pt-4">
          <div class="w-16 h-16 rounded-2xl bg-slate-200 dark:bg-slate-800 overflow-hidden border-2 border-white dark:border-slate-700 shadow-lg">
            <img src="https://github.com/identicons/app.png" class="w-full h-full object-cover" />
          </div>
          <div>
            <h3 class="text-xl font-black text-slate-800 dark:text-slate-100">YourName</h3>
            <p class="text-sm text-slate-500 font-medium">Independent Developer</p>
            <div class="flex gap-3 mt-2">
              <a href="#" class="text-slate-400 hover:text-primary transition-colors"><Github class="w-4 h-4" /></a>
              <a href="#" class="text-slate-400 hover:text-blue-400 transition-colors"><Twitter class="w-4 h-4" /></a>
              <a href="#" class="text-slate-400 hover:text-emerald-400 transition-colors"><Mail class="w-4 h-4" /></a>
            </div>
          </div>
        </CardContent>
      </Card>

      <Card class="md:col-span-7 border-none shadow-xl shadow-slate-200/50 dark:shadow-none bg-slate-900 text-white rounded-[2.5rem] relative overflow-hidden">
        <Scale class="absolute -right-6 -bottom-6 w-32 h-32 opacity-10 -rotate-12" />
        <CardContent class="p-8 flex flex-col justify-between h-full">
          <div>
            <div class="flex items-center gap-2 mb-4">
              <Badge class="bg-emerald-500 hover:bg-emerald-500 text-white border-none px-3 py-0.5 rounded-full font-black text-[10px]">MIT LICENSE</Badge>
            </div>
            <h3 class="text-lg font-bold mb-2">完全开源且自由</h3>
            <p class="text-slate-400 text-sm leading-relaxed">
              Favordex 采用 MIT 协议开源。您可以自由地使用、修改、分发本软件，甚至用于商业用途。
            </p>
          </div>
          <Button variant="link" class="text-emerald-400 p-0 h-auto self-start font-bold mt-4 hover:no-underline hover:text-emerald-300">
            阅读协议全文 →
          </Button>
        </CardContent>
      </Card>

      <Card class="md:col-span-12 border-none shadow-xl shadow-slate-200/50 dark:shadow-none bg-white/60 dark:bg-slate-900/60 backdrop-blur-xl rounded-[2.5rem]">
        <CardHeader class="flex flex-row items-center justify-between">
          <CardTitle class="text-xs font-black uppercase tracking-widest text-slate-400 flex items-center gap-2">
            <Users2 class="w-4 h-4" /> Contributors
          </CardTitle>
          <span class="text-[10px] font-bold text-slate-400 bg-slate-100 dark:bg-slate-800 px-3 py-1 rounded-full">8 Persons Joined</span>
        </CardHeader>
        <CardContent class="p-8 pt-0">
          <div class="flex flex-wrap gap-4">
            <div v-for="i in 8" :key="i" class="group relative">
              <div class="w-12 h-12 rounded-xl bg-slate-100 dark:bg-slate-800 border-2 border-transparent group-hover:border-primary transition-all cursor-pointer overflow-hidden shadow-sm">
                <img :src="`https://api.dicebear.com/7.x/avataaars/svg?seed=${i}`" class="w-full h-full object-cover" />
              </div>
              <span class="absolute -top-10 left-1/2 -translate-x-1/2 px-2 py-1 bg-slate-800 text-white text-[10px] rounded opacity-0 group-hover:opacity-100 transition-opacity whitespace-nowrap">Contributor #{{ i }}</span>
            </div>
            <button class="w-12 h-12 rounded-xl border-2 border-dashed border-slate-300 dark:border-slate-700 flex items-center justify-center text-slate-400 hover:border-primary hover:text-primary transition-all">
              <Plus class="w-5 h-5" />
            </button>
          </div>
        </CardContent>
      </Card>

      <Card class="md:col-span-12 border-none shadow-xl shadow-slate-200/50 dark:shadow-none bg-white/40 dark:bg-slate-900/40 rounded-[2.5rem] border border-white dark:border-slate-800">
        <CardContent class="p-6 flex flex-wrap items-center justify-center gap-x-10 gap-y-4 opacity-50 grayscale hover:grayscale-0 transition-all duration-500">
          <div class="flex items-center gap-2 font-black text-sm"><Cpu class="w-4 h-4" /> Tauri Framework</div>
          <div class="flex items-center gap-2 font-black text-sm"><Code2 class="w-4 h-4" /> Rust Core</div>
          <div class="flex items-center gap-2 font-black text-sm"><Zap class="w-4 h-4" /> Vue.js 3</div>
          <div class="flex items-center gap-2 font-black text-sm"><ShieldCheck class="w-4 h-4" /> OpenAI SDK</div>
        </CardContent>
      </Card>
    </div>

    <div class="flex flex-col items-center gap-6">
      <div class="flex gap-4">
        <Button class="rounded-2xl px-10 h-14 font-black shadow-xl shadow-primary/20 hover:scale-105 transition-transform active:scale-95">
          <RefreshCw class="w-4 h-4 mr-2" /> 检查更新
        </Button>
        <Button variant="outline" class="rounded-2xl px-10 h-14 border-2 font-black bg-white/50 dark:bg-slate-800/50 backdrop-blur-md">
          <Github class="w-4 h-4 mr-2" /> 参与贡献
        </Button>
      </div>
      <p class="text-[10px] font-bold text-slate-400 uppercase tracking-widest">© 2026 Favordex Project. Built with Love and Rust.</p>
    </div>

  </div>
</template>

<script setup lang="ts">
import {
  Bookmark, User, Users2, Scale, Github, Twitter, Mail, 
  Plus, Cpu, Code2, Zap, ShieldCheck, RefreshCw
} from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
</script>

<style scoped>
/* 可以在这里添加一些特殊的动画 */
</style>