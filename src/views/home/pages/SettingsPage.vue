<template>
    <div
        class="p-8 h-screen w-full flex flex-col gap-8 bg-slate-50/30 dark:bg-slate-950 overflow-hidden space-y-10 animate-in slide-in-from-bottom-6 duration-500 max-w-2xl">
        <header>
            <h1 class="text-4xl font-black text-slate-800 dark:text-slate-100 tracking-tight">偏好设置</h1>
            <p class="text-slate-500 mt-2 font-medium">管理您的整理配置</p>
        </header>

        <div class="space-y-6 animate-in slide-in-from-right-8 duration-500">
            <div class="space-y-4">
                <div class="flex items-center justify-between">
                    <Label class="text-[11px] font-black uppercase text-slate-400 tracking-widest">AI 供应商</Label>
                    <div class="flex p-1 bg-slate-100 dark:bg-slate-800 rounded-xl">
                        <button v-for="m in (['openai', 'ollama', 'google'] as const)" :key="m"
                            @click="handleSupplierChange(m)"
                            :class="['px-4 py-1.5 text-[10px] font-black rounded-lg transition-all uppercase',
                                preferences.aiSupplier.supplier === m ? 'bg-white dark:bg-slate-700 text-primary shadow-sm scale-105' : 'text-slate-400 hover:text-slate-600']">
                            {{ m }}
                        </button>
                    </div>
                </div>

                <div class="grid gap-3">
                    <div class="group flex items-center gap-3 px-4 py-3 bg-slate-50 dark:bg-slate-800/50 rounded-2xl border border-transparent focus-within:border-primary/30 focus-within:bg-white dark:focus-within:bg-slate-900 transition-all">
                        <Network class="w-4 h-4 text-slate-400 group-focus-within:text-primary transition-colors" />
                        <input v-model="preferences.aiSupplier.apiUrl" placeholder="Endpoint URL"
                            class="bg-transparent text-sm w-full outline-none font-medium" />
                    </div>

                    <div v-if="preferences.aiSupplier.supplier !== 'ollama'"
                        class="group flex items-center gap-3 px-4 py-3 bg-slate-50 dark:bg-slate-800/50 rounded-2xl border border-transparent focus-within:border-primary/30 focus-within:bg-white dark:focus-within:bg-slate-900 transition-all">
                        <KeyRound class="w-4 h-4 text-slate-400 group-focus-within:text-primary transition-colors" />
                        <input v-model="preferences.aiSupplier.apiKey" type="password" placeholder="API Key"
                            class="bg-transparent text-sm w-full outline-none font-medium" />
                    </div>

                    <Button v-if="!isVerified" variant="outline" size="sm"
                        class="rounded-xl border-primary/20 text-primary hover:bg-primary/5 font-bold h-11"
                        @click="verifyAI" :disabled="isVerifying">
                        <Loader2 v-if="isVerifying" class="mr-2 h-4 w-4 animate-spin" />
                        {{ isVerifying ? '正在建立连接...' : '连接并发现模型' }}
                    </Button>

                    <div v-else class="flex items-center gap-3 px-4 py-3 bg-green-500/10 border border-green-500/20 rounded-2xl animate-in slide-in-from-top-2">
                        <Zap class="w-4 h-4 text-green-500 fill-green-500" />
                        <Select v-model="preferences.aiSupplier.model">
                            <SelectTrigger class="w-full bg-transparent border-none shadow-none focus:ring-0 px-0">
                                <SelectValue placeholder="请选择模型" />
                            </SelectTrigger>
                            <SelectContent>
                                <SelectGroup>
                                    <SelectLabel>可用模型</SelectLabel>
                                    <SelectItem v-for="m in availableModels" :key="m" :value="m">{{ m }}</SelectItem>
                                </SelectGroup>
                            </SelectContent>
                        </Select>
                    </div>
                </div>
            </div>

            <div class="space-y-2 border-t border-slate-100 dark:border-slate-800 pt-4">
                <div class="flex items-center justify-between p-3 rounded-2xl hover:bg-slate-50 dark:hover:bg-slate-800/50 transition-colors group">
                    <div class="flex items-center gap-3">
                        <LayoutGrid class="w-4 h-4 text-slate-400" />
                        <Label class="text-sm font-bold cursor-pointer text-slate-700 dark:text-slate-300">
                            <AnimatedTooltip :tip="'启用后，将使用默认分类体系对书签进行分类。'">默认分类体系</AnimatedTooltip>
                        </Label>
                    </div>
                    <Switch v-model="preferences.smartCategorize" />
                </div>

                <div class="flex items-center justify-between p-3 rounded-2xl hover:bg-slate-50 dark:hover:bg-slate-800/50 transition-colors group">
                    <div class="flex items-center gap-3">
                        <Trash2 class="w-4 h-4 text-slate-400" />
                        <Label class="text-sm font-bold cursor-pointer text-slate-700 dark:text-slate-300">
                            <AnimatedTooltip :tip="'启用后，将自动删除书签中的失效项。'">清理失效书签</AnimatedTooltip>
                        </Label>
                    </div>
                    <Switch v-model="preferences.automaticallyDeleteInvalidatedBookmarks" />
                </div>

                <div class="flex items-center justify-between p-3 rounded-2xl hover:bg-slate-50 dark:hover:bg-slate-800/50 transition-colors group">
                    <div class="flex items-center gap-3">
                        <Globe class="w-4 h-4 text-slate-400" />
                        <Label class="text-sm font-bold cursor-pointer text-slate-700 dark:text-slate-300">
                            <AnimatedTooltip :tip="'启用后，将使用系统代理进行请求，通常你需要配置代理服务器。'">全局代理</AnimatedTooltip>
                        </Label>
                    </div>
                    <Switch v-model="preferences.systemProxy" />
                </div>
            </div>

            <div class="flex items-center justify-between px-3">
                <span class="text-[11px] font-black uppercase text-slate-400 tracking-widest flex items-center gap-2">
                    <Clock class="w-3 h-3" /> <AnimatedTooltip :tip="'AI 请求的超时时间(ms)。建议 10000 以上。'">请求超时</AnimatedTooltip>
                </span>
                <div class="flex items-center gap-2 px-3 py-1 bg-slate-100 dark:bg-slate-800 rounded-lg">
                    <input v-model.number="preferences.timeout" type="number" class="w-22 text-center bg-transparent text-xs font-bold outline-none" />
                    <span class="text-[10px] font-bold text-slate-400 uppercase">ms</span>
                </div>
            </div>

            <div class="flex items-center justify-between px-3">
                <span class="text-[11px] font-black uppercase text-slate-400 tracking-widest flex items-center gap-2">
                    <Clock class="w-3 h-3" /> <AnimatedTooltip :tip="'最大同时处理的 AI 任务数量。'">最大任务数</AnimatedTooltip>
                </span>
                <div class="flex items-center gap-2 px-3 py-1 bg-slate-100 dark:bg-slate-800 rounded-lg">
                    <input v-model.number="preferences.max_tasks" type="number" class="w-12 text-center bg-transparent text-xs font-bold outline-none" />
                    <span class="text-[10px] font-bold text-slate-400 uppercase">个</span>
                </div>
            </div>
        </div>

        <div class="flex items-center px-3">
            <Button variant="ghost" size="sm" @click="reset" class="text-slate-400 hover:text-primary transition-colors">
                <RotateCcw class="w-4 h-4 mr-1" /> 恢复默认
            </Button>
            <div class="flex-1"></div>
            <Button variant="default" size="sm" class="rounded-xl border-primary/20 font-bold h-11"
                :disabled="!isVerified" @click="saveAndFinish">保存并完成</Button>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { Trash2, Network, KeyRound, LayoutGrid, Globe, RotateCcw, Zap, Clock, Loader2 } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import { Switch } from '@/components/ui/switch'
import { Label } from '@/components/ui/label'
import { AnimatedTooltip } from '@/components/ui/animated-tooltip'
import { Select, SelectContent, SelectGroup, SelectItem, SelectLabel, SelectTrigger, SelectValue } from '@/components/ui/select'
import StorageUtil from '@/utils/storageUtil'
import { getAiModels } from '@/api'
import { toast } from 'vue-sonner'

// --- 类型定义 ---
interface AiSupplier {
    supplier: 'openai' | 'ollama' | 'google'
    apiUrl: string
    apiKey: string
    model: string
}

interface Preferences {
    smartCategorize: boolean
    aiSupplier: AiSupplier
    automaticallyDeleteInvalidatedBookmarks: boolean
    timeout: number
    max_tasks: number
    systemProxy: boolean
    isInit: boolean
}

// --- 常量配置 ---
const DEFAULT_SUPPLIERS: Record<AiSupplier['supplier'], AiSupplier> = {
    openai: {
        supplier: 'openai',
        apiUrl: 'https://api.openai.com/v1',
        apiKey: import.meta.env.VITE_OPENAI_KEY || '',
        model: '',
    },
    ollama: {
        supplier: 'ollama',
        apiUrl: 'http://localhost:11434/',
        apiKey: '',
        model: '',
    },
    google: {
        supplier: 'google',
        apiUrl: 'https://generativelanguage.googleapis.com/',
        apiKey: import.meta.env.VITE_GOOGLE_API_KEY || '',
        model: '',
    }
}

// --- 响应式状态 ---
const isVerifying = ref(false)
const isVerified = ref(false)
const availableModels = ref<string[]>([])

// 使用函数获取初始默认值
const getInitialPreferences = (): Preferences => ({
    smartCategorize: true,
    aiSupplier: { ...DEFAULT_SUPPLIERS.openai },
    automaticallyDeleteInvalidatedBookmarks: false,
    timeout: 10000,
    max_tasks: 3,
    systemProxy: true,
    isInit: false,
})

const preferences = ref<Preferences>(getInitialPreferences())

// --- 核心方法 ---

// 从存储同步数据
const syncFromStorage = () => {
    // 使用 ?? 确保 false 值能被正确识别，而不是回退到默认的 true
    preferences.value.smartCategorize = StorageUtil.get<boolean>('smartCategorize') ?? true
    preferences.value.automaticallyDeleteInvalidatedBookmarks = StorageUtil.get<boolean>('automaticallyDeleteInvalidatedBookmarks') ?? false
    preferences.value.systemProxy = StorageUtil.get<boolean>('systemProxy') ?? true
    preferences.value.timeout = StorageUtil.get<number>('timeout') ?? 10000
    preferences.value.max_tasks = StorageUtil.get<number>('max_tasks') ?? 3
    
    const storedAi = StorageUtil.get<AiSupplier>('aiSupplier')
    if (storedAi) {
        preferences.value.aiSupplier = storedAi
        if (storedAi.model) {
            isVerified.value = true
            availableModels.value = [storedAi.model]
        }
    }
}

// 切换供应商逻辑
const handleSupplierChange = (supplier: AiSupplier['supplier']) => {
    isVerified.value = false
    availableModels.value = []
    // 切换到对应供应商的默认配置
    preferences.value.aiSupplier = { ...DEFAULT_SUPPLIERS[supplier] }
}

// 验证 AI
const verifyAI = async () => {
    isVerifying.value = true
    try {
        const { supplier, apiUrl, apiKey } = preferences.value.aiSupplier
        const models = await getAiModels(supplier, apiUrl, apiKey)
        
        if (models && models.length > 0) {
            availableModels.value = models
            preferences.value.aiSupplier.model = models[0]
            isVerified.value = true
            toast.success(`连接成功，找到 ${models.length} 个可用模型`)
        } else {
            throw new Error('未发现可用模型')
        }
    } catch (err: any) {
        toast.error(`验证失败: ${err.message || '网络连接异常'}`)
        isVerified.value = false
    } finally {
        isVerifying.value = false
    }
}

// 保存
const saveAndFinish = () => {
    const p = preferences.value
    StorageUtil.set('aiSupplier', p.aiSupplier)
    StorageUtil.set('smartCategorize', p.smartCategorize)
    StorageUtil.set('automaticallyDeleteInvalidatedBookmarks', p.automaticallyDeleteInvalidatedBookmarks)
    StorageUtil.set('systemProxy', p.systemProxy)
    StorageUtil.set('timeout', p.timeout)
    StorageUtil.set('max_tasks', p.max_tasks)
    StorageUtil.set('isInit', true)
    toast.success('配置已保存')
}

// 重置
const reset = () => {
    preferences.value = getInitialPreferences()
    isVerified.value = false
    availableModels.value = []
    // 立即清除存储并写入默认
    saveAndFinish()
    toast.info('已恢复默认配置')
}

onMounted(syncFromStorage)
</script>