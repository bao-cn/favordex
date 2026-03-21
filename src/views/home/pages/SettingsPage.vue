<template>
    <div class="space-y-10 animate-in slide-in-from-bottom-6 duration-500 max-w-2xl">
        <header>
            <h1 class="text-4xl font-black text-slate-800 dark:text-slate-100 tracking-tight">偏好设置</h1>
            <p class="text-slate-500 mt-2 font-medium">自定义您的 AI 整理引擎行为</p>
        </header>

        <div class="space-y-8">
            <Card
                class="border-none shadow-2xl shadow-slate-200/50 dark:shadow-none bg-white/80 dark:bg-slate-900/80 backdrop-blur-xl rounded-[2.5rem] overflow-hidden">
                <CardContent class="p-8 space-y-8">
                    <div class="flex items-center justify-between">
                        <Label class="text-[12px] font-black uppercase text-slate-500 tracking-widest">AI 服务商</Label>
                        <div class="flex p-1.5 bg-slate-100 dark:bg-slate-800 rounded-2xl border border-slate-200/30">
                            <button v-for="m in ['openai', 'ollama']" :key="m"
                                :class="['px-5 py-2 text-[10px] font-black rounded-xl transition-all uppercase',
                                    activeProvider === m ? 'bg-white dark:bg-slate-700 text-primary shadow-sm scale-105' : 'text-slate-400 hover:text-slate-600']"
                                @click="activeProvider = m">
                                {{ m }}
                            </button>
                        </div>
                    </div>
                    <div class="space-y-4">
                        <div
                            class="group flex items-center gap-4 px-5 py-4 bg-slate-50 dark:bg-slate-800/50 rounded-2xl border-2 border-transparent focus-within:border-primary/30 focus-within:bg-white dark:focus-within:bg-slate-900 transition-all">
                            <Network class="w-5 h-5 text-slate-400 group-focus-within:text-primary transition-colors" />
                            <input placeholder="Endpoint URL" value="https://api.openai.com/v1"
                                class="bg-transparent text-sm w-full outline-none font-bold text-slate-700 dark:text-slate-200" />
                        </div>
                        <div
                            class="group flex items-center gap-4 px-5 py-4 bg-slate-50 dark:bg-slate-800/50 rounded-2xl border-2 border-transparent focus-within:border-primary/30 focus-within:bg-white dark:focus-within:bg-slate-900 transition-all">
                            <KeyRound
                                class="w-5 h-5 text-slate-400 group-focus-within:text-primary transition-colors" />
                            <input type="password" placeholder="API Key" value="sk-..........."
                                class="bg-transparent text-sm w-full outline-none font-bold text-slate-700 dark:text-slate-200" />
                        </div>
                    </div>
                </CardContent>
            </Card>

            <Card
                class="border-none shadow-2xl shadow-slate-200/50 dark:shadow-none bg-white/80 dark:bg-slate-900/80 backdrop-blur-xl rounded-[2.5rem] overflow-hidden">
                <CardContent class="p-4">
                    <div
                        class="flex items-center justify-between p-5 rounded-3xl hover:bg-slate-50 dark:hover:bg-slate-800/50 transition-colors">
                        <div class="flex items-center gap-5">
                            <div class="p-3 rounded-2xl bg-blue-50 dark:bg-blue-900/20 text-blue-500">
                                <LayoutGrid class="w-5 h-5" />
                            </div>
                            <div>
                                <Label
                                    class="text-sm font-black text-slate-700 dark:text-slate-200 block">启用智能分类</Label>
                                <span class="text-xs text-slate-400 mt-1 block">AI 将根据网页内容自动建立文件夹</span>
                            </div>
                        </div>
                        <Switch :checked="true" />
                    </div>
                    <div
                        class="flex items-center justify-between p-5 rounded-3xl hover:bg-slate-50 dark:hover:bg-slate-800/50 transition-colors">
                        <div class="flex items-center gap-5">
                            <div class="p-3 rounded-2xl bg-rose-50 dark:bg-rose-900/20 text-rose-500">
                                <Trash2 class="w-5 h-5" />
                            </div>
                            <div>
                                <Label
                                    class="text-sm font-black text-slate-700 dark:text-slate-200 block">自动清理失效书签</Label>
                                <span class="text-xs text-slate-400 mt-1 block">整理时自动移除非法或404状态的链接</span>
                            </div>
                        </div>
                        <Switch :checked="false" />
                    </div>
                </CardContent>
            </Card>
            <Button class="w-full h-16 rounded-3xl font-black text-lg shadow-2xl shadow-primary/20">保存所有更改</Button>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import {
    Trash2, Network, KeyRound, LayoutGrid,
} from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import { Card, CardContent } from '@/components/ui/card'
import { Switch } from '@/components/ui/switch'
import { Label } from '@/components/ui/label'

const activeProvider = ref('openai')
</script>

<style scoped></style>