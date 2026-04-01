<template>
    <div class="p-8 h-screen w-full flex flex-col gap-8 bg-slate-50/30 dark:bg-slate-950 overflow-hidden">

        <header class="flex justify-between items-end flex-none">
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

        <Card
            class="p-0 pt-7 gap-0 flex-1 min-h-0 border-none shadow-2xl shadow-slate-200/50 bg-white dark:bg-slate-900/70 backdrop-blur-xl rounded-[2.5rem] overflow-hidden flex flex-col">
            <CardHeader class="flex-none border-b border-slate-100 dark:border-slate-800">
                <CardTitle class="text-lg font-black flex items-center gap-3 text-slate-400">
                    <div class="p-2 rounded-lg bg-primary/10 text-primary">
                        <FolderTree class="w-4 h-4" />
                    </div>
                    {{ activeBrowser || '未选择' }} 书签结构
                    <div class="flex-1"></div>
                    <div class="flex flex-row items-center gap-2">
                        <p class="text-xs font-black uppercase text-blue-500 tracking-[0.2em]">总计书签</p>
                        <p class="text-lg font-black text-slate-800 dark:text-slate-100">{{ stats.total }}</p>
                    </div>
                    <Button variant="outline" :disabled="!activeBrowser" @click="start()">
                        <Sparkles class="w-5 h-5 text-yellow-400 fill-yellow-400" /> 开始整理
                    </Button>
                </CardTitle>
            </CardHeader>

            <CardContent class="flex-1 min-h-0 p-0 flex overflow-hidden">
                <aside
                    class="w-80 flex-none border-r border-slate-100 dark:border-slate-800 bg-slate-50/50 dark:bg-slate-950/20 overflow-y-auto p-4">
                    <ElTree :data="bookmarkTree" :props="defaultProps" :default-expand-all="true"
                        :empty-text="activeBrowser ? '暂无书签' : '请选择浏览器'" @node-click="handleNodeClick"
                        class="custom-el-tree bg-transparent">
                        <template #default="{ data }">
                            <div class="flex items-center gap-2 py-1 truncate">
                                <Folder class="w-4 h-4 text-amber-400 fill-amber-400/20" />
                                <span
                                    class="text-sm font-bold truncate text-slate-600 dark:text-slate-300">{{ data.name }}</span>
                            </div>
                        </template>
                    </ElTree>
                </aside>

                <main class="flex-1 flex flex-col min-w-0 bg-white/40 dark:bg-transparent">
                    <div v-if="folderBookmarks.length === 0"
                        class="flex-1 flex flex-col items-center justify-center text-slate-400/40">
                        <Bookmark class="w-16 h-16 mb-4 stroke-1" />
                        <p class="font-bold italic">点击目录查看书签详情</p>
                    </div>

                    <div v-else class="flex-1 p-6 overflow-hidden">
                        <RecycleScroller class="h-full pr-4" :items="folderBookmarks" :item-size="96" key-field="url">
                            <template #default="{ item }">
                                <div
                                    class="group flex items-center justify-between p-4 mb-1 bg-white dark:bg-slate-900 rounded-2xl border border-slate-100 dark:border-slate-800 shadow-sm hover:shadow-md transition-all">
                                    <div class="flex items-center gap-4 min-w-0">
                                        <div
                                            class="w-10 h-10 rounded-lg bg-slate-50 dark:bg-slate-800 flex-none flex items-center justify-center border border-slate-100 dark:border-slate-700 overflow-hidden">
                                            <img :src="`https://www.google.com/s2/favicons?domain=${(item as IBookmark).url}&sz=64`"
                                                class="w-5 h-5 object-contain" />
                                        </div>
                                        <div class="min-w-0">
                                            <h4 class="font-bold text-slate-800 dark:text-slate-200 truncate">
                                                {{ (item as IBookmark).name }}</h4>
                                            <p class="text-xs text-slate-400 truncate font-medium">
                                                {{ (item as IBookmark).url }}</p>
                                        </div>
                                    </div>
                                    <Button variant="ghost" size="icon"
                                        @click="handleBookmarkClick((item as IBookmark).url)"
                                        class="flex-none rounded-xl opacity-0 group-hover:opacity-100 bg-slate-50 dark:bg-slate-800">
                                        <Link2 class="w-4 h-4" />
                                    </Button>
                                </div>
                            </template>
                        </RecycleScroller>
                    </div>
                </main>
            </CardContent>
        </Card>

        <Dialog v-model:open="processDialogVisible">
            <DialogContent class="sm:max-w-162.5 rounded-3xl!" @pointer-down-outside.prevent @escape-key-down.prevent :show-close-button="false">
                <DialogHeader>
                    <DialogTitle class="flex items-center gap-2">
                        <template v-if="isProcessing">
                            <Loader2 class="w-5 h-5 text-blue-500 animate-spin" />
                            AI 正在整理书签...
                        </template>
                        <template v-else>
                            <Sparkles class="w-5 h-5 text-yellow-500 fill-yellow-500" />
                            整理完成预览
                        </template>
                    </DialogTitle>
                </DialogHeader>

                <div v-if="isProcessing" class="py-12 flex flex-col items-center justify-center gap-6">
                    <div class="relative w-36 h-36 flex items-center justify-center">
                        <svg class="absolute inset-0 w-full h-full -rotate-90" viewBox="0 0 100 100">
                            <circle class="text-slate-100 dark:text-slate-800 stroke-current" stroke-width="8" cx="50"
                                cy="50" r="40" fill="transparent"></circle>
                            <circle class="text-blue-500 stroke-current transition-all duration-300 ease-out"
                                stroke-width="8" stroke-linecap="round" cx="50" cy="50" r="40" fill="transparent"
                                :stroke-dasharray="251.2" :stroke-dashoffset="progress && progress.total > 0
                                    ? 251.2 - (251.2 * progress.current) / progress.total
                                    : 251.2
                                    ">
                            </circle>
                        </svg>
                        <span
                            class="text-3xl font-black text-slate-700 dark:text-slate-200">{{ Math.round((progress?.current || 0) / (progress?.total || 1) * 100) }}%</span>
                    </div>
                    <p class="text-sm font-bold text-slate-500 animate-pulse">{{ processStatus }}</p>
                </div>

                <div v-else class="flex flex-col gap-4 py-2">
                    <p class="text-sm text-slate-500 font-medium">AI 已经完成了分类整理，以下是即将生成的书签目录树。确认无误后可覆盖浏览器原书签：</p>
                    <div
                        class="h-87.5 overflow-y-auto border border-slate-200 dark:border-slate-800 rounded-2xl p-4 bg-slate-50/50 dark:bg-slate-900/50">
                        <ElTree :data="previewTreeData" :props="defaultProps" :default-expand-all="true"
                            class="custom-el-tree bg-transparent">
                            <template #default="{ data }">
                                <div class="flex items-center gap-2 py-1 truncate">
                                    <Folder v-if="data.type === 'folder'"
                                        class="w-4 h-4 text-emerald-400 fill-emerald-400/20" />
                                    <Link2 v-else class="w-4 h-4 text-yellow-400 fill-yellow-400/20" />
                                    <span class="text-sm font-bold text-slate-600">{{ data.name }}
                                        <span v-if="data.type === 'url'"
                                            class="text-xs text-slate-400 ml-2">{{ data.url }}</span>
                                    </span>
                                    <span v-if="data.children && data.type === 'folder'"
                                        class="ml-1 text-[10px] px-2 py-0.5 rounded-full bg-slate-200 dark:bg-slate-800 text-slate-500 font-bold">
                                        {{ data.children.length }} 项
                                    </span>
                                </div>
                            </template>
                        </ElTree>
                    </div>
                </div>

                <DialogFooter v-if="!isProcessing">
                    <Button variant="outline" @click="processDialogVisible = false" class="rounded-xl">取消</Button>
                    <Button @click="handleConfirmOverwrite"
                        class="rounded-xl bg-blue-500 hover:bg-blue-600 text-white shadow-lg shadow-blue-500/30">
                        <CheckCircle2 class="w-4 h-4 mr-2" /> 确认覆盖书签
                    </Button>
                </DialogFooter>
            </DialogContent>
        </Dialog>
        <Dialog v-model:open="confirmDialogVisible">
            <DialogContent class="sm:max-w-md rounded-3xl!">
                <DialogHeader>
                    <DialogTitle class="flex items-center gap-2 text-amber-500">
                        <AlertTriangle class="w-5 h-5" /> 整理确认
                    </DialogTitle>
                </DialogHeader>

                <div class="py-4 space-y-5">
                    <div>
                        <div
                            class="flex items-start gap-3 p-3 rounded-2xl border bg-slate-50/50 dark:bg-slate-900/50 border-slate-100 dark:border-slate-800">
                            <div class="text-blue-500">
                                <Info class="w-4 h-4" />
                            </div>
                            <span
                                class="text-xs font-bold text-slate-700 dark:text-slate-300">整理过程受到 AI 推理速度影响，当书签过多时速度会很慢，建议在低峰期进行操作</span>
                        </div>
                    </div>
                    <div>
                        <div v-for="(item, index) in confirmConfigs" :key="index"
                            class="flex items-start gap-3 p-3 rounded-2xl border bg-slate-50/50 dark:bg-slate-900/50 border-slate-100 dark:border-slate-800">
                            <div :class="item.type === 'warning' ? 'text-amber-500' : 'text-blue-500'">
                                <AlertCircle v-if="item.type === 'warning'" class="w-4 h-4" />
                                <Info v-else class="w-4 h-4" />
                            </div>
                            <span class="text-xs font-bold text-slate-700 dark:text-slate-300">{{ item.title }}</span>
                        </div>
                    </div>

                    <div v-if="!StorageUtil.get('smartCategorize')"
                        class="space-y-3 p-4 rounded-2xl bg-primary/2 border border-primary/10 flex flex-col">
                        <div class="flex items-center gap-2">
                            <LayoutGrid class="w-4 h-4 text-primary" />
                            <span class="text-sm font-black text-slate-700 dark:text-slate-200">选择目标分类体系</span>
                        </div>
                        <p class="text-[11px] text-slate-500">AI 将根据您选定的体系结构对书签进行归类</p>

                        <Select v-model="selectedSystemId">
                            <SelectTrigger
                                class="w-full bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 text-sm font-bold outline-none">
                                <SelectValue placeholder="请选择分类体系" />
                            </SelectTrigger>
                            <SelectContent>
                                <SelectItem v-for="sys in availableSystems" :key="sys.id" :value="sys.id">
                                    {{ sys.name }}
                                </SelectItem>
                            </SelectContent>
                        </Select>
                    </div>

                    <p class="text-[11px] text-slate-400 italic text-center">注意：书签覆盖操作不可逆，请确保已了解风险</p>
                </div>

                <DialogFooter class="sm:justify-center gap-2">
                    <Button variant="outline" @click="confirmDialogVisible = false" class="rounded-xl flex-1 font-bold">
                        取消
                    </Button>
                    <Button @click="handleProceedToProcess"
                        class="rounded-xl flex-1 font-bold bg-slate-900 dark:bg-slate-100 dark:text-slate-900 shadow-xl">
                        确认无误，开始整理
                    </Button>
                </DialogFooter>
            </DialogContent>
        </Dialog>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import {
    FolderTree, Bookmark, Sparkles, Folder, Link2, LayoutGrid,
    Loader2, CheckCircle2, AlertTriangle, AlertCircle, Info
} from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import { Select, SelectTrigger, SelectValue, SelectContent, SelectItem } from '@/components/ui/select'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogFooter } from '@/components/ui/dialog'
import { ElTree } from 'element-plus'
import { open } from '@tauri-apps/plugin-shell'
import { getBookmarkFolders, getBookmarksByFolder, getBookmarksNum, checkBackup, listenToOrganizeProgress, getAllBookmarks, organizeBookmarks } from '@/api'
import { Ebrowser, IBookmarkFolder, IBookmark, IClassificationTask, IClassifyOptions, IProgressPayload } from '@/api'
import { RecycleScroller } from 'vue-virtual-scroller'
import StorageUtil from '@/utils/storageUtil'
import { toast } from 'vue-sonner'

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

// === 新增整理与预览弹窗的相关状态 ===
const processDialogVisible = ref(false)
const isProcessing = ref(false)
const progress = ref<IProgressPayload | null>(null)
const processStatus = ref('')
const previewTreeData = ref<IBookmarkFolder[]>([])
const confirmDialogVisible = ref(false)
const confirmConfigs = ref<{ title: string; type: 'warning' | 'info' }[]>([])
let stopListen: () => void;

import classification from '@/classification.json'

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
        console.log(res)
        bookmarkTree.value = res
    })
    await getBookmarksNum(browser).then((res: number) => {
        stats.value.total = res
    })
}

const loadBookmarksByFolder = async (browser: Ebrowser, folderId: number) => {
    await getBookmarksByFolder(browser, folderId).then((res: IBookmark[]) => {
        folderBookmarks.value = res
    })
}

const selectedSystemId = ref('default')
const availableSystems = ref<{ id: string; name: string }[]>([])

const start = () => {
    if (!StorageUtil.get<boolean>('isInit')) {
        toast.error('请进入设置页面，完成模型设置')
        return
    }
    if (!StorageUtil.get<boolean>('userAuthorization')) {
        toast.error('您未授权书签访问权限，无法使用该功能')
        return
    }

    const warnings: { title: string; type: 'warning' | 'info' }[] = []

    if (!checkBackup(browsersIndicator)) {
        warnings.push({ title: '您尚未备份当前书签，建议操作前手动备份', type: 'warning' })
    }
    if (StorageUtil.get<boolean>('automaticallyDeleteInvalidatedBookmarks')) {
        warnings.push({ title: '已启用“自动删除失效书签”，清理后无法找回', type: 'info' })
    }

    const isSmart = StorageUtil.get<boolean>('smartCategorize')
    if (isSmart) {
        warnings.push({ title: '已启用“默认分类”，将按照默认分类体系进行整理', type: 'info' })
    } else {
        const loaded: { id: string; name: string }[] = [{ id: 'default', name: '系统默认分类' }]
        const STORAGE_PREFIX = 'CATEGORY_SYS_'
        for (let i = 0; i < localStorage.length; i++) {
            const key = localStorage.key(i)
            if (key && key.startsWith(STORAGE_PREFIX)) {
                const sysData = StorageUtil.get<{ id: string, name: string }>(key)
                if (sysData) loaded.push({ id: sysData.id, name: sysData.name })
            }
        }
        availableSystems.value = loaded
    }

    confirmConfigs.value = warnings
    confirmDialogVisible.value = true
}

const handleProceedToProcess = async () => {
    confirmDialogVisible.value = false
    processDialogVisible.value = true
    isProcessing.value = true

    processStatus.value = `AI 正在整理书签，该操作需要一些时间，请喝一杯咖啡稍作等待...`

    const task: IClassificationTask = {
        title: '',
        url: '',
        taxonomy: classification,
    }

    interface AiSupplier {
        supplier: 'openai' | 'ollama' | 'google'
        apiUrl: string
        apiKey: string
        model: string
    }
    const ai = StorageUtil.get<AiSupplier>('aiSupplier')
    const option: IClassifyOptions = {
        auto_sort_local: true,
        auto_sort_dead: !StorageUtil.get<boolean>('automaticallyDeleteInvalidatedBookmarks'),
        provider: ai?.supplier || 'google',
        api_key: ai?.apiKey || import.meta.env.VITE_GOOGLE_API_KEY || '',
        model: ai?.model || 'gemma-3-12b-it',
        auto_delete: StorageUtil.get<boolean>('automaticallyDeleteInvalidatedBookmarks') || false,
        system_prompt: StorageUtil.get<string>('systemPrompt') || undefined,
        system_proxy: StorageUtil.get<boolean>('systemProxy') || true,
        timeout_secs: StorageUtil.get<number>('timeoutSec') || 10,
        max_tasks: StorageUtil.get<number>('maxTasks') || 3,
    }

    stopListen = await listenToOrganizeProgress((data) => {
        progress.value = data;
    });

    await organizeBookmarks(browsersIndicator, task, option).then((res: IBookmarkFolder[]) => {

        console.log(res)
        const resStr = JSON.stringify(res)
        console.log(resStr)
        previewTreeData.value = res;
    })

    await new Promise(r => setTimeout(r, 1000))
    processStatus.value = '整理完成！'
    await new Promise(r => setTimeout(r, 1000))

    setTimeout(() => {
        isProcessing.value = false
    }, 500)

    if (stopListen) stopListen();
}

const handleConfirmOverwrite = async () => {

    // TODO: 这里调用覆盖 API
    toast.success(`成功覆盖 ${activeBrowser.value} 浏览器的书签！`)
    processDialogVisible.value = false

}

</script>

<style scoped>
.vue-recycle-scroller {
    height: 100%;
}

:deep(.el-tree) {
    background: transparent !important;
}

.overflow-y-auto::-webkit-scrollbar {
    width: 5px;
}

.overflow-y-auto::-webkit-scrollbar-thumb {
    background: rgba(0, 0, 0, 0.1);
    border-radius: 10px;
}

.dark .overflow-y-auto::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.1);
}
</style>