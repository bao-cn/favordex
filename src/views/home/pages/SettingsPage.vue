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
                            @click="preferences.aiSupplier.supplier = m"
                            :class="['px-4 py-1.5 text-[10px] font-black rounded-lg transition-all uppercase',
                                preferences.aiSupplier.supplier === m ? 'bg-white dark:bg-slate-700 text-primary shadow-sm scale-105' : 'text-slate-400 hover:text-slate-600']">
                            {{ m }}
                        </button>
                    </div>
                </div>

                <div class="grid gap-3">
                    <div
                        class="group flex items-center gap-3 px-4 py-3 bg-slate-50 dark:bg-slate-800/50 rounded-2xl border border-transparent focus-within:border-primary/30 focus-within:bg-white dark:focus-within:bg-slate-900 transition-all">
                        <Network class="w-4 h-4 text-slate-400 group-focus-within:text-primary transition-colors" />
                        <input v-model="preferences.aiSupplier.apiUrl" placeholder="Endpoint URL"
                            class="bg-transparent text-sm w-full outline-none font-medium" />
                    </div>

                    <div v-if="!(preferences.aiSupplier.supplier === 'ollama')"
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

                    <div v-else
                        class="flex items-center gap-3 px-4 py-3 bg-green-500/10 border border-green-500/20 rounded-2xl animate-in slide-in-from-top-2">
                        <Zap
                            class="w-4 h-4 text-green-500 fill-green-500 group-focus-within:text-primary transition-colors" />
                        <Select v-model="preferences.aiSupplier.model" class="bg-white dark:bg-slate-800">
                            <SelectTrigger class="w-full bg-white">
                                <SelectValue placeholder="请选择模型" />
                            </SelectTrigger>
                            <SelectContent>
                                <SelectGroup>
                                    <SelectLabel>模型列表</SelectLabel>
                                    <SelectItem v-for="m in availableModels" :key="m" :value="m">
                                        {{ m }}
                                    </SelectItem>
                                </SelectGroup>
                            </SelectContent>
                        </Select>
                    </div>
                </div>
            </div>

            <div class="space-y-2 border-t border-slate-100 dark:border-slate-800 pt-4">
                <div
                    class="flex items-center justify-between p-3 rounded-2xl hover:bg-slate-50 dark:hover:bg-slate-800/50 transition-colors group">
                    <div class="flex items-center gap-3">
                        <div>
                            <component :is="LayoutGrid" class="w-4 h-4" />
                        </div>
                        <Label class="text-sm font-bold cursor-pointer text-slate-700 dark:text-slate-300">智能分类</Label>
                    </div>
                    <Switch v-model="smartCategorizeClone" />
                </div>
                <div
                    class="flex items-center justify-between p-3 rounded-2xl hover:bg-slate-50 dark:hover:bg-slate-800/50 transition-colors group">
                    <div class="flex items-center gap-3">
                        <div>
                            <component :is="Trash2" class="w-4 h-4" />
                        </div>
                        <Label
                            class="text-sm font-bold cursor-pointer text-slate-700 dark:text-slate-300">清理失效书签</Label>
                    </div>
                    <Switch v-model="automaticallyDeleteInvalidatedBookmarksClone" />
                </div>
                <div
                    class="flex items-center justify-between p-3 rounded-2xl hover:bg-slate-50 dark:hover:bg-slate-800/50 transition-colors group">
                    <div class="flex items-center gap-3">
                        <div>
                            <component :is="Globe" class="w-4 h-4" />
                        </div>
                        <Label class="text-sm font-bold cursor-pointer text-slate-700 dark:text-slate-300">全局代理</Label>
                    </div>
                    <Switch v-model="systemProxyClone" />
                </div>
            </div>

            <div class="flex items-center justify-between px-3">
                <span class="text-[11px] font-black uppercase text-slate-400 tracking-widest flex items-center gap-2">
                    <Clock class="w-3 h-3" /> 超时时间
                </span>
                <div class="flex items-center gap-2 px-3 py-1 bg-slate-100 dark:bg-slate-800 rounded-lg">
                    <input v-model="preferences.timeout" type="number"
                        class="w-22 text-center bg-transparent text-xs font-bold outline-none" />
                    <span class="text-[10px] font-bold text-slate-400 uppercase">ms</span>
                </div>
            </div>
        </div>
        <div class="flex items-center px-3">
            <Button variant="ghost" size="sm" @click="reset"
                class="text-slate-400 hover:text-primary transition-colors">
                <RotateCcw class="w-4 h-4" /> 恢复默认
            </Button>
            <div class="flex-1"></div>
            <Button variant="default" size="sm" class="rounded-xl border-primary/20 font-bold h-11"
                :disabled="!isVerified" @click="saveAndFinish">保存并完成</Button>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted } from 'vue'
import {
    Trash2, Network, KeyRound, LayoutGrid, Globe, RotateCcw, Zap, Clock
} from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import StorageUtil from '@/utils/storageUtil'
import { Switch } from '@/components/ui/switch'
import { Label } from '@/components/ui/label'
import {
    Select,
    SelectContent,
    SelectGroup,
    SelectItem,
    SelectLabel,
    SelectTrigger,
    SelectValue,
} from '@/components/ui/select'
import { getAiModels } from '@/api'
import { toast } from 'vue-sonner'

const isVerifying = ref(false)              // 是否正在验证
const isVerified = ref(false)               // 是否验证通过
const availableModels = ref<string[]>([])   // 可用模型列表

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
    systemProxy: boolean
    userAuthorization: boolean
    isInit: boolean
}

const OpenAISupplier: AiSupplier = {
    supplier: 'openai',
    apiUrl: 'https://api.openai.com/v1',
    apiKey: import.meta.env.VITE_OPENAI_KEY || '',
    model: '',
}

const OllamaSupplier: AiSupplier = {
    supplier: 'ollama',
    apiUrl: 'http://localhost:11434/',
    apiKey: '',
    model: '',
}

const GoogleSupplier: AiSupplier = {
    supplier: 'google',
    apiUrl: 'https://generativelanguage.googleapis.com/',
    apiKey: import.meta.env.VITE_GOOGLE_API_KEY || '',
    model: '',
}



const smartCategorizeClone = ref(StorageUtil.get<boolean>('smartCategorize') || true)
const automaticallyDeleteInvalidatedBookmarksClone = ref(StorageUtil.get<boolean>('automaticallyDeleteInvalidatedBookmarks') || false)
const systemProxyClone = ref(StorageUtil.get<boolean>('systemProxy') || true)

const preferences = ref<Preferences>({
    smartCategorize: true,
    aiSupplier: OpenAISupplier,
    automaticallyDeleteInvalidatedBookmarks: true,
    timeout: 10000,
    systemProxy: true,
    userAuthorization: false,
    isInit: false,
})

const initLocalStorage = () => {
    StorageUtil.set<boolean>('smartCategorize', true)
    StorageUtil.set<AiSupplier>('aiSupplier', OpenAISupplier)
    StorageUtil.set<boolean>('automaticallyDeleteInvalidatedBookmarks', false)
    StorageUtil.set<number>('timeout', 10000)
    StorageUtil.set<boolean>('systemProxy', true)
    StorageUtil.set<boolean>('userAuthorization', false)
    StorageUtil.set<boolean>('isInit', false)
}



const syncFromStorage = () => {
    const ai = StorageUtil.get<AiSupplier>('aiSupplier') || OpenAISupplier
    preferences.value = {
        smartCategorize: StorageUtil.get<boolean>('smartCategorize') || true,
        aiSupplier: {
            supplier: ai.supplier,
            apiUrl: ai.apiUrl,
            apiKey: ai.apiKey,
            model: ai.model,
        },
        automaticallyDeleteInvalidatedBookmarks: StorageUtil.get<boolean>('automaticallyDeleteInvalidatedBookmarks') || true,
        timeout: StorageUtil.get<number>('timeout') || 10000,
        systemProxy: StorageUtil.get<boolean>('systemProxy') || true,
        userAuthorization: StorageUtil.get<boolean>('userAuthorization') || false,
        isInit: StorageUtil.get<boolean>('isInit') || false,
    }
    smartCategorizeClone.value = StorageUtil.get<boolean>('smartCategorize') || true
    automaticallyDeleteInvalidatedBookmarksClone.value = StorageUtil.get<boolean>('automaticallyDeleteInvalidatedBookmarks') || false
    systemProxyClone.value = StorageUtil.get<boolean>('systemProxy') || true
    if (preferences.value.aiSupplier.model) {
        isVerified.value = true
        availableModels.value = [preferences.value.aiSupplier.model]
    }
}

const verifyAI = async () => {
    isVerifying.value = true
    console.log(preferences.value)
    availableModels.value = await getAiModels(preferences.value.aiSupplier.supplier, preferences.value.aiSupplier.apiUrl, preferences.value.aiSupplier.apiKey).then(res => {
        isVerified.value = true
        isVerifying.value = false
        preferences.value.aiSupplier.model = availableModels.value[0]
        toast.success(`找到${availableModels.value.length}个可用模型`)
        return res
    }).catch(err => {
        toast.error(`验证 AI 模型失败: ${err.message}`)
        isVerifying.value = false
        return []
    })

}

const reset = () => {
    initLocalStorage()
    syncFromStorage()
}

const saveAndFinish = () => {
    StorageUtil.set<AiSupplier>('aiSupplier', {
        supplier: preferences.value.aiSupplier.supplier,
        model: preferences.value.aiSupplier.model,
        apiUrl: preferences.value.aiSupplier.apiUrl,
        apiKey: preferences.value.aiSupplier.apiKey,
    })
    StorageUtil.set<boolean>('smartCategorize', smartCategorizeClone.value)
    StorageUtil.set<boolean>('automaticallyDeleteInvalidatedBookmarks', automaticallyDeleteInvalidatedBookmarksClone.value)
    StorageUtil.set<boolean>('systemProxy', systemProxyClone.value)
    StorageUtil.set<number>('timeout', preferences.value.timeout)
    StorageUtil.set<boolean>('isInit', true)
    toast.success('设置已保存')
}

onMounted(() => {
    syncFromStorage()
})



watch(() => preferences.value.aiSupplier.supplier, (newVal) => {
    isVerified.value = false

    switch (newVal) {
        case OpenAISupplier.supplier:
            preferences.value.aiSupplier.model = OpenAISupplier.model
            preferences.value.aiSupplier.apiUrl = OpenAISupplier.apiUrl
            preferences.value.aiSupplier.apiKey = OpenAISupplier.apiKey
            break;
        case OllamaSupplier.supplier:
            preferences.value.aiSupplier.model = OllamaSupplier.model
            preferences.value.aiSupplier.apiUrl = OllamaSupplier.apiUrl
            preferences.value.aiSupplier.apiKey = OllamaSupplier.apiKey
            break;
        case GoogleSupplier.supplier:
            preferences.value.aiSupplier.model = GoogleSupplier.model
            preferences.value.aiSupplier.apiUrl = GoogleSupplier.apiUrl
            preferences.value.aiSupplier.apiKey = GoogleSupplier.apiKey
            break;
        default:
            break;
    }
})
</script>

<style scoped></style>