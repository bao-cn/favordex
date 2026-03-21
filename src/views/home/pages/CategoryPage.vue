<template>
    <div class="space-y-8 animate-in slide-in-from-bottom-6 duration-500">
        <header class="flex justify-between items-end">
            <div>
                <h1 class="text-4xl font-black text-slate-800 dark:text-slate-100 tracking-tight">分类管理</h1>
                <p class="text-slate-500 mt-2 text-sm font-medium">定义 AI 整理时使用的分类体系</p>
            </div>
            <Button class="rounded-2xl h-12 px-6 font-bold shadow-xl shadow-primary/20">
                <Plus class="w-4 h-4 mr-2" /> 新建分类
            </Button>
        </header>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
            <Card class="border-2 border-slate-100 dark:border-slate-800 shadow-none bg-transparent rounded-4xl">
                <CardHeader class="p-8 pb-4">
                    <CardTitle class="text-base font-black flex items-center gap-2 text-slate-700 dark:text-slate-200">
                        <ShieldCheck class="w-5 h-5 text-primary" /> 官方推荐分类
                    </CardTitle>
                </CardHeader>
                <CardContent class="p-8 pt-0 space-y-3">
                    <div v-for="cat in officialCategories" :key="cat"
                        class="px-5 py-4 bg-white dark:bg-slate-900 rounded-2xl border border-slate-100 dark:border-slate-800 text-sm font-bold text-slate-600 dark:text-slate-300 shadow-sm flex items-center justify-between hover:border-primary/30 transition-colors">
                        {{ cat }}
                        <Lock class="w-3.5 h-3.5 text-slate-300 dark:text-slate-600" />
                    </div>
                </CardContent>
            </Card>

            <Card
                class="border-none shadow-2xl shadow-slate-200/50 dark:shadow-none bg-white/80 dark:bg-slate-900/80 backdrop-blur-xl rounded-4xl">
                <CardHeader class="p-8 pb-4 border-b border-slate-100 dark:border-slate-800">
                    <CardTitle class="text-base font-black flex items-center gap-2 text-slate-700 dark:text-slate-200">
                        <UserCog class="w-5 h-5 text-purple-500" /> 我的自定义分类
                    </CardTitle>
                </CardHeader>
                <CardContent class="p-6 space-y-1">
                    <div v-for="cat in customCategoryTree" :key="cat.id"
                        class="flex items-center gap-2 py-3 px-4 rounded-xl hover:bg-slate-50 dark:hover:bg-slate-800/50 transition-colors group"
                        :style="{ paddingLeft: `${(cat.level * 1.5) + 1}rem` }">
                        <GripVertical
                            class="w-4 h-4 text-slate-300 opacity-0 group-hover:opacity-100 cursor-grab transition-opacity" />
                        <Hash class="w-4 h-4 text-purple-400" />
                        <span class="text-sm font-bold text-slate-700 dark:text-slate-300">{{ cat.name }}</span>
                        <div class="ml-auto opacity-0 group-hover:opacity-100 transition-opacity flex gap-2">
                            <Button variant="ghost" size="icon"
                                class="w-8 h-8 rounded-lg hover:text-blue-500 hover:bg-blue-50">
                                <Edit2 class="w-3.5 h-3.5" />
                            </Button>
                            <Button variant="ghost" size="icon"
                                class="w-8 h-8 rounded-lg hover:text-red-500 hover:bg-red-50">
                                <Trash2 class="w-3.5 h-3.5" />
                            </Button>
                        </div>
                    </div>
                </CardContent>
            </Card>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import {
  Plus, ShieldCheck, Lock, UserCog, GripVertical, Hash, Edit2, Trash2,
} from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'

const officialCategories = ref(['技术开发', '设计素材', '日常工具', '影音娱乐', '学习资料'])
const customCategoryTree = ref([
  { id: 1, name: '前端生态圈', level: 0 },
  { id: 2, name: 'Vue 相关的库', level: 1 },
  { id: 3, name: 'UI 组件库', level: 1 },
  { id: 4, name: '独立开发必备', level: 0 },
  { id: 5, name: '出海支付平台', level: 1 },
])
</script>

<style scoped></style>