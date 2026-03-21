<template>
    <div class="space-y-10 animate-in fade-in slide-in-from-bottom-6 duration-700 h-screen">
        <header class="flex justify-between items-end">
            <div class="space-y-1">
                <h1 class="text-4xl font-black tracking-tight text-slate-800 dark:text-slate-100">书签总览</h1>
                <p class="text-slate-500 dark:text-slate-400 font-medium">选择浏览器并开始您的书签焕新之旅</p>
            </div>
            <div
                class="flex gap-4 p-1.5 bg-slate-100 dark:bg-slate-800/50 rounded-2xl border border-slate-200/50 dark:border-slate-700/50 backdrop-blur-md">
                <Button variant="ghost"
                    class="rounded-xl font-bold hover:bg-white dark:hover:bg-slate-700 shadow-none transition-all"
                    @click="loadBookmarks(Ebrowser.Chrome)">
                    <img src="@/assets/chrome.svg" alt="Chrome" class="w-4 h-4 mr-2" /> 加载 Chrome
                </Button>
                <Button variant="ghost"
                    class="rounded-xl font-bold hover:bg-white dark:hover:bg-slate-700 shadow-none transition-all"
                    @click="loadBookmarks(Ebrowser.Edge)">
                    <img src="@/assets/edge.png" alt="Edge" class="w-4 h-4 mr-2" /> 加载 Edge
                </Button>
            </div>
        </header>

        <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
            <Card
                class="group border-none shadow-2xl shadow-slate-200/50 dark:shadow-none bg-white/80 dark:bg-slate-900/70 backdrop-blur-xl rounded-[2.5rem] transition-all hover:-translate-y-1">
                <CardContent class="p-8 flex items-center justify-between">
                    <div class="space-y-1">
                        <p class="text-xs font-black uppercase text-blue-500 tracking-[0.2em]">总计书签</p>
                        <p class="text-5xl font-black text-slate-800 dark:text-slate-100">{{ stats.total }}</p>
                    </div>
                    <div
                        class="w-16 h-16 rounded-3xl bg-blue-500/10 flex items-center justify-center text-blue-600 transition-colors group-hover:bg-blue-500 group-hover:text-white">
                        <Bookmark class="w-8 h-8" />
                    </div>
                </CardContent>
            </Card>

            <Card
                class="group border-none shadow-2xl shadow-slate-200/50 dark:shadow-none bg-white/80 dark:bg-slate-900/70 backdrop-blur-xl rounded-[2.5rem] transition-all hover:-translate-y-1">
                <CardContent class="p-8 flex items-center justify-between">
                    <div class="space-y-1">
                        <p class="text-xs font-black uppercase text-rose-500 tracking-[0.2em]">失效链接</p>
                        <p class="text-5xl font-black text-rose-500">{{ stats.invalid }}</p>
                    </div>
                    <div
                        class="w-16 h-16 rounded-3xl bg-rose-500/10 flex items-center justify-center text-rose-500 transition-colors group-hover:bg-rose-500 group-hover:text-white">
                        <Unlink class="w-8 h-8" />
                    </div>
                </CardContent>
            </Card>

            <div class="flex flex-col gap-6">
                <Button variant="outline" :disabled="!activeBrowser"
                    class="flex-1 h-20 rounded-3xl text-lg font-black border-2 border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900 hover:bg-slate-50 dark:hover:bg-slate-800 transition-all active:scale-95 text-slate-700 dark:text-slate-200">
                    <Sparkles class="w-6 h-6 mr-3 text-yellow-300 fill-yellow-300" /> 开始 AI 智能整理
                </Button>
                <Button variant="outline" :disabled="!activeBrowser"
                    class="flex-1 h-20 rounded-3xl text-lg font-black border-2 border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900 hover:bg-slate-50 dark:hover:bg-slate-800 transition-all active:scale-95 text-slate-700 dark:text-slate-200">
                    <ShieldAlert class="w-6 h-6 mr-3 text-rose-500" /> 分析失效书签
                </Button>
            </div>
        </div>

        <Card
            class="border-none shadow-2xl shadow-slate-200/50 dark:shadow-none bg-white dark:bg-slate-900/70 backdrop-blur-xl rounded-[2.5rem] overflow-hidden h-full">
            <CardHeader class="">
                <CardTitle class="text-lg font-black flex items-center gap-3 text-slate-400">
                    <div class="p-2 rounded-lg bg-primary/10 text-primary">
                        <FolderTree class="w-4 h-4" />
                    </div>
                    {{ activeBrowser || '未选择' }} 书签结构
                </CardTitle>
            </CardHeader>
            <CardContent class="p-0">
                <div class="flex border-t border-slate-100 dark:border-slate-800">
                    <aside
                        class="w-80 border-r border-slate-100 dark:border-slate-800 p-6 overflow-y-auto bg-slate-50/50 dark:bg-slate-950/20">
                        <ElTree :data="bookmarkTree" :props="defaultProps" :default-expand-all="true"
                            :expand-on-click-node="false" @node-click="handleNodeClick" class="custom-el-tree">
                            <template #default="{ data }">
                                <div class="flex items-center gap-2 py-1 group/tree truncate">
                                    <Folder
                                        class="w-4 h-4 text-amber-400 fill-amber-400/20 group-hover/tree:fill-amber-400/40 transition-colors" />
                                    <span
                                        class="text-sm font-bold truncate text-slate-600 dark:text-slate-300">{{ data.name }}</span>
                                </div>
                            </template>
                        </ElTree>
                    </aside>
                    <div class="flex-1 p-8 overflow-y-auto space-y-4 bg-white/50 dark:bg-transparent">
                        <div v-if="folderBookmarks.length === 0"
                            class="flex flex-col items-center justify-center text-slate-400/40 italic">
                            <Bookmark class="w-16 h-16 mb-4 stroke-1" />
                            <p class="font-bold">点击目录查看书签详情</p>
                        </div>
                        <RecycleScroller :items="folderBookmarks" :item-size="120" :key-field="'url'">
                            <template #default="{ item }">
                                <div
                                    class="group flex items-center justify-between p-5 bg-white dark:bg-slate-900 rounded-[1.25rem] border border-slate-100 dark:border-slate-800/60 shadow-sm hover:shadow-xl hover:shadow-slate-200/50 dark:hover:shadow-none transition-all hover:border-primary/40 hover:-translate-y-0.5">
                                    <div class="flex items-center gap-5 min-w-0">
                                        <div
                                            class="w-12 h-12 rounded-xl bg-slate-50 dark:bg-slate-800 shrink-0 flex items-center justify-center border border-slate-100 dark:border-slate-700 overflow-hidden">
                                            <img :src="`https://www.google.com/s2/favicons?domain=${(item as IBookmark).url}&sz=64`"
                                                class="w-6 h-6 object-contain" loading="lazy" />
                                        </div>
                                        <div class="min-w-0">
                                            <h4
                                                class="font-black text-slate-800 dark:text-slate-200 truncate leading-tight">
                                                {{ (item as IBookmark).name }}</h4>
                                            <p class="text-xs text-slate-400 truncate mt-1 font-medium">
                                                {{ (item as IBookmark).url }}</p>
                                        </div>
                                    </div>
                                    <Button variant="ghost" size="icon"
                                        @click="handleBookmarkClick((item as IBookmark).url)"
                                        class="rounded-xl opacity-0 group-hover:opacity-100 transition-opacity bg-slate-50 dark:bg-slate-800 hover:text-primary">
                                        <Link2 class="w-4 h-4" />
                                    </Button>
                                </div>
                            </template>
                        </RecycleScroller>

                    </div>
                </div>
            </CardContent>
        </Card>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import {
    FolderTree, Bookmark,
    Sparkles, Unlink, ShieldAlert, Folder, Link2
} from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { ElTree } from 'element-plus'
import { open } from '@tauri-apps/plugin-shell'
// --- 业务代码核心保留 ---
import { getBookmarkFolders, getBookmarksByFolder, getBookmarksNum } from '@/api'
import { Ebrowser, IBookmarkFolder, IBookmark } from '@/api'
import { RecycleScroller } from 'vue-virtual-scroller'
defineExpose({ Ebrowser })

let browsersIndicator = Ebrowser.Chrome
const activeBrowser = ref('')
const stats = ref({ total: 0, invalid: 0 })
const bookmarkTree = ref<IBookmarkFolder[]>([])
const folderBookmarks = ref<IBookmark[]>([])
const defaultProps = {
    children: 'children',
    label: 'name',
}

const handleNodeClick = (node: IBookmarkFolder) => {
    loadBookmarksByFolder(browsersIndicator, node.id)
}

const handleBookmarkClick = async (url: string) => {
    await open(url)
}

const loadBookmarks = async (browser: Ebrowser) => {
    browsersIndicator = browser
    activeBrowser.value = browser.charAt(0).toUpperCase() + browser.slice(1)
    await getBookmarkFolders(browser).then((res: IBookmarkFolder[]) => {
        bookmarkTree.value = res
    })
    await getBookmarksNum(browser).then((res: number) => {
        stats.value.total = res
    })
}

const loadBookmarksByFolder = async (browser: Ebrowser, folderId: string) => {
    await getBookmarksByFolder(browser, folderId).then((res: IBookmark[]) => {
        folderBookmarks.value = res
    })
}
</script>

<style scoped></style>