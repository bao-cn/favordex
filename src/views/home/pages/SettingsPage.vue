<template>
  <div class="flex h-screen w-full bg-background/50 dark:bg-slate-950 overflow-hidden">
    <div class="w-64 border-r border-border bg-slate-50/50 dark:bg-slate-900/20 p-6 flex flex-col gap-8">
      <header>
        <h1 class="text-2xl font-bold tracking-tight">偏好设置</h1>
        <p class="text-xs text-muted-foreground mt-1 font-medium">Favordex 配置中心</p>
      </header>

      <nav class="flex flex-col gap-1">
        <button v-for="tab in navigation" :key="tab.id" @click="activeTab = tab.id" :class="[
          'flex items-center gap-3 px-3 py-2.5 rounded-xl text-sm font-semibold transition-all',
          activeTab === tab.id
            ? 'bg-primary text-primary-foreground shadow-md shadow-primary/20 scale-[1.02]'
            : 'text-slate-500 hover:bg-slate-100 dark:hover:bg-slate-800 hover:text-slate-900'
        ]">
          <component :is="tab.icon" class="w-4 h-4" />
          {{ tab.name }}
        </button>
      </nav>

      <div class="mt-auto">
        <Button variant="ghost" size="sm" @click="reset"
          class="w-full justify-start text-slate-400 hover:text-destructive gap-2 px-3 rounded-xl">
          <RotateCcw class="w-4 h-4" />
          恢复默认配置
        </Button>
      </div>
    </div>

    <main class="flex-1 overflow-y-auto relative scroll-smooth">
      <div class="max-w-3xl mx-auto p-12 space-y-10 pb-32">
        
        <section v-show="activeTab === 'general'" class="animate-in fade-in slide-in-from-bottom-4 duration-500">
          <div class="mb-6">
            <h2 class="text-lg font-bold">通用与自动化</h2>
            <p class="text-sm text-muted-foreground">控制书签的智能分类与维护逻辑</p>
          </div>
          
          <div class="grid gap-4">
            <Card class="overflow-hidden border-none shadow-sm bg-slate-50/50 dark:bg-slate-900/40">
              <div class="divide-y divide-border/40">
                <div class="flex items-center justify-between p-4 group transition-colors hover:bg-slate-100/50 dark:hover:bg-slate-800/30">
                  <div class="space-y-0.5">
                    <Label class="text-sm font-bold flex items-center gap-2">
                      <LayoutGrid class="w-4 h-4 text-primary" /> 默认分类体系
                    </Label>
                    <p class="text-xs text-muted-foreground">启用后使用系统默认分类体系进行分类</p>
                  </div>
                  <Switch v-model="preferences.smartCategorize" />
                </div>

                <div class="flex items-center justify-between p-4 group transition-colors hover:bg-slate-100/50 dark:hover:bg-slate-800/30">
                  <div class="space-y-0.5">
                    <Label class="text-sm font-bold flex items-center gap-2">
                      <Trash2 class="w-4 h-4 text-primary" /> 清理失效书签
                    </Label>
                    <p class="text-xs text-muted-foreground">自动检测并移除无法访问的链接</p>
                  </div>
                  <Switch v-model="preferences.automaticallyDeleteInvalidatedBookmarks" />
                </div>
              </div>
            </Card>
          </div>
        </section>

        <section v-show="activeTab === 'ai'" class="animate-in fade-in slide-in-from-bottom-4 duration-500">
          <div class="mb-6">
            <h2 class="text-lg font-bold">AI 供应商</h2>
            <p class="text-sm text-muted-foreground">选择并配置为您提供处理能力的推理引擎</p>
          </div>

          <Card class="p-6 space-y-6 border-none shadow-sm bg-slate-50/50 dark:bg-slate-900/40">
            <div class="space-y-4">
              <Label class="text-[11px] font-black uppercase text-muted-foreground tracking-widest">选择供应商</Label>
              <div class="flex p-1 bg-slate-200/50 dark:bg-slate-800 rounded-xl w-fit">
                <button v-for="m in (['openai', 'ollama', 'google'] as const)" :key="m"
                  @click="handleSupplierChange(m)"
                  :class="['px-5 py-2 text-xs font-bold rounded-lg transition-all uppercase',
                    preferences.aiSupplier.supplier === m ? 'bg-white dark:bg-slate-700 text-primary shadow-sm scale-105' : 'text-slate-500 hover:text-slate-700']">
                  {{ m }}
                </button>
              </div>
            </div>

            <div class="grid gap-4">
              <div class="space-y-2">
                <Label class="text-xs font-bold ml-1">接口地址 (Endpoint)</Label>
                <div class="relative group">
                  <Network class="absolute left-4 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground group-focus-within:text-primary transition-colors" />
                  <input v-model="preferences.aiSupplier.apiUrl" placeholder="https://api.example.com/v1"
                    class="flex h-11 w-full rounded-xl border border-input bg-background px-11 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary/20 transition-all" />
                </div>
              </div>

              <div v-if="preferences.aiSupplier.supplier !== 'ollama'" class="space-y-2">
                <Label class="text-xs font-bold ml-1">API 密钥 (Authentication)</Label>
                <div class="relative group">
                  <KeyRound class="absolute left-4 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground group-focus-within:text-primary transition-colors" />
                  <input v-model="preferences.aiSupplier.apiKey" type="password" placeholder="sk-••••••••••••"
                    class="flex h-11 w-full rounded-xl border border-input bg-background px-11 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary/20 transition-all" />
                </div>
              </div>

              <div class="pt-2">
                <Button v-if="!isVerified" variant="outline" class="w-full rounded-xl font-bold h-11 border-primary/20 text-primary hover:bg-primary/5 shadow-sm"
                  @click="verifyAI" :disabled="isVerifying">
                  <Loader2 v-if="isVerifying" class="mr-2 h-4 w-4 animate-spin" />
                  {{ isVerifying ? '正在验证接口能力...' : '验证并自动匹配模型' }}
                </Button>

                <div v-else class="flex items-center gap-4 p-4 bg-green-500/5 border border-green-500/10 rounded-2xl animate-in zoom-in-95">
                  <div class="bg-green-500 p-1.5 rounded-full shadow-lg shadow-green-500/20">
                    <Zap class="w-3.5 h-3.5 text-white fill-white" />
                  </div>
                  <div class="flex-1">
                    <Select v-model="preferences.aiSupplier.model">
                      <SelectTrigger class="w-full bg-transparent border-none shadow-none focus:ring-0 px-0 h-auto font-bold py-0 text-green-600 dark:text-green-400">
                        <SelectValue placeholder="请选择模型" />
                      </SelectTrigger>
                      <SelectContent>
                        <SelectGroup>
                          <SelectLabel>发现的可用模型</SelectLabel>
                          <SelectItem v-for="m in availableModels" :key="m" :value="m">{{ m }}</SelectItem>
                        </SelectGroup>
                      </SelectContent>
                    </Select>
                  </div>
                </div>
              </div>
            </div>
          </Card>
        </section>

        <section v-show="activeTab === 'network'" class="animate-in fade-in slide-in-from-bottom-4 duration-500">
          <div class="mb-6">
            <h2 class="text-lg font-bold">网络与性能</h2>
            <p class="text-sm text-muted-foreground">调整连接限制与代理服务器</p>
          </div>

          <Card class="p-6 space-y-8 border-none shadow-sm bg-slate-50/50 dark:bg-slate-900/40">
            <div class="flex items-center justify-between">
              <div class="space-y-0.5">
                <Label class="text-sm font-bold flex items-center gap-2">
                  <Globe class="w-4 h-4 text-primary" /> 全局系统代理
                </Label>
                <p class="text-xs text-muted-foreground">遵循操作系统的全局代理配置进行请求</p>
              </div>
              <Switch v-model="preferences.systemProxy" />
            </div>

            <div class="space-y-6 pt-2">
              <div class="flex items-center justify-between">
                <div class="space-y-0.5">
                  <Label class="text-sm font-bold">请求超时限制</Label>
                  <p class="text-xs text-muted-foreground">单次网络请求的最大等待时间 (ms)</p>
                </div>
                <div class="flex items-center gap-2 px-3 py-1.5 bg-background border rounded-xl shadow-inner">
                  <input v-model.number="preferences.timeout" type="number"
                    class="w-16 text-center bg-transparent text-xs font-bold outline-none" />
                  <span class="text-[10px] font-bold text-muted-foreground">MS</span>
                </div>
              </div>

              <div class="flex items-center justify-between">
                <div class="space-y-0.5">
                  <Label class="text-sm font-bold">并发处理上限</Label>
                  <p class="text-xs text-muted-foreground">同时运行的 AI 处理任务最大数量</p>
                </div>
                <div class="flex items-center gap-2 px-3 py-1.5 bg-background border rounded-xl shadow-inner">
                  <input v-model.number="preferences.max_tasks" type="number"
                    class="w-16 text-center bg-transparent text-xs font-bold outline-none" />
                  <span class="text-[10px] font-bold text-muted-foreground">TASKS</span>
                </div>
              </div>
            </div>
          </Card>
        </section>

      </div>

      <div class="absolute bottom-0 left-0 right-0 p-6 bg-linear-to-t from-background via-background to-transparent pointer-events-none">
        <div class="max-w-3xl mx-auto flex justify-end pointer-events-auto">
          <Button variant="default" size="lg" class="rounded-2xl px-12 font-black shadow-xl shadow-primary/20 h-12 transition-all hover:scale-[1.02] active:scale-[0.98]"
            :disabled="!isVerified" @click="saveAndFinish">
            保存全部修改
          </Button>
        </div>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { 
  Trash2, Network, KeyRound, LayoutGrid, Globe, RotateCcw, 
  Zap, Loader2, Settings, Cpu 
} from 'lucide-vue-next'
import { Card } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Switch } from '@/components/ui/switch'
import { Label } from '@/components/ui/label'
import { Select, SelectContent, SelectGroup, SelectItem, SelectLabel, SelectTrigger, SelectValue } from '@/components/ui/select'
import StorageUtil from '@/utils/storageUtil'
import { getAiModels } from '@/api'
import { toast } from 'vue-sonner'

const activeTab = ref('general')
const navigation = [
  { id: 'general', name: '通用设置', icon: Settings },
  { id: 'ai', name: '模型引擎', icon: Cpu },
  { id: 'network', name: '网络性能', icon: Globe },
]

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