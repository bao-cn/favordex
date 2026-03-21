<template>
  <div
    class="min-h-screen bg-[#f8fafc] dark:bg-[#020617] flex flex-col items-center justify-center p-6 antialiased selection:bg-primary/10">

    <header class="w-full max-w-md mb-12">
      <div class="relative flex justify-between">
        <div class="absolute top-5 left-1 w-[calc(100%-10px)] h-0.5 bg-slate-200 dark:bg-slate-800 z-0">
          <div class="h-full bg-primary transition-all duration-500 ease-in-out" :style="{ width: progress + '%' }">
          </div>
        </div>

        <div v-for="(step, i) in steps" :key="i" class="relative z-10 flex flex-col items-center gap-2">
          <div :class="[
            'w-10 h-10 rounded-full border-2 flex items-center justify-center transition-all duration-500',
            currentStep > i ? 'bg-primary border-primary text-white scale-90' :
              currentStep === i ? 'bg-white dark:bg-slate-900 border-primary text-primary shadow-[0_0_15px_rgba(var(--primary),0.3)]' :
                'bg-white dark:bg-slate-900 border-slate-200 dark:border-slate-800 text-slate-400'
          ]">
            <Check v-if="currentStep > i" class="w-5 h-5" />
            <component v-else :is="step.icon" class="w-5 h-5" />
          </div>
          <span
            :class="['text-[10px] font-bold uppercase tracking-widest transition-colors duration-300', currentStep >= i ? 'text-primary' : 'text-slate-400']">
            {{ step.title }}
          </span>
        </div>
      </div>
    </header>

    <main class="w-full max-w-lg">
      <transition name="card-slide" mode="out-in">
        <Card :key="currentStep"
          class="border-none shadow-[0_20px_50px_rgba(0,0,0,0.05)] bg-white/80 dark:bg-slate-900/80 backdrop-blur-xl rounded-4xl overflow-hidden">
          <CardHeader class="pt-10 pb-4 text-center">
            <CardTitle class="text-2xl font-black tracking-tight text-slate-800 dark:text-slate-100">
              {{ steps[currentStep].desc }}
            </CardTitle>
          </CardHeader>

          <CardContent class="min-h-105 px-10 flex flex-col justify-center">

            <div v-if="currentStep === 0" class="space-y-8 animate-in fade-in zoom-in-95 duration-500">
              <div class="relative mx-auto w-24 h-24">
                <div class="absolute inset-0 bg-primary/20 rounded-full animate-ping"></div>
                <div
                  class="relative bg-linear-to-tr from-primary to-blue-400 w-full h-full rounded-full flex items-center justify-center text-white shadow-lg">
                  <Sparkles class="w-10 h-10" />
                </div>
              </div>
              <div class="text-center space-y-3">
                <h3 class="text-lg font-bold">准备好焕新您的书签了吗？</h3>
                <p class="text-slate-500 text-sm leading-relaxed">
                  失效链接清理与 AI 驱动的书签整理，只需几步配置即可开始
                </p>
              </div>
            </div>

            <div v-if="currentStep === 1"
              class="space-y-6 animate-in slide-in-from-right-8 duration-500 flex flex-col items-center">
              <div
                class="group p-6 rounded-3xl border-2 border-dashed border-slate-200 dark:border-slate-800 hover:border-primary/50 transition-all bg-slate-50/50 dark:bg-slate-800/30">
                <div class="flex flex-col items-center text-center gap-4">
                  <div
                    class="p-4 bg-white dark:bg-slate-900 rounded-2xl shadow-sm text-primary group-hover:scale-110 transition-transform">
                    <ShieldCheck class="w-8 h-8" />
                  </div>
                  <div>
                    <h4 class="font-bold text-slate-800 dark:text-slate-200">修改书签权限</h4>
                    <p class="text-xs text-slate-500 mt-1">软件将操作您的书签，建议点击“备份书签”备份您的书签</p>
                  </div>
                  <Button :variant="preferences.userAuthorization ? 'outline' : 'default'" @click="handleAuthorize"
                    :disabled="preferences.userAuthorization"
                    class="w-full rounded-xl py-6 font-bold uppercase tracking-wider">
                    <Loader2 v-if="isAuthorizing" class="mr-2 h-4 w-4 animate-spin" />
                    {{ preferences.userAuthorization ? '权限已就绪' : '立即授权' }}
                  </Button>
                </div>
              </div>
              <Dialog>
                <DialogTrigger>
                  <Button variant="ghost" class="text-xs font-bold uppercase tracking-widest">备份书签</Button>
                </DialogTrigger>
                <DialogContent>
                  <DialogHeader>
                    <DialogTitle>选择浏览器</DialogTitle>
                    <DialogDescription>
                      请选择您已安装的浏览器，以备份书签
                    </DialogDescription>
                    <div class="flex items-center gap-2.5">
                      <Button variant="outline" :disabled="!browsers.chrome" @click="handleBackupBookmarks(Ebrowser.Chrome)">
                        <img :src="Chrome" alt="Chrome" class="w-4 h-4" />
                        Google Chrome
                      </Button>
                      <Button variant="outline" :disabled="!browsers.edge" @click="handleBackupBookmarks(Ebrowser.Edge)">
                        <img :src="Edge" alt="Edge" class="w-4 h-4" />
                        Microsoft Edge
                      </Button>
                    </div>
                  </DialogHeader>
                </DialogContent>
              </Dialog>
            </div>

            <div v-if="currentStep === 2" class="space-y-6 animate-in slide-in-from-right-8 duration-500">

              <div class="space-y-4">
                <div class="flex items-center justify-between">
                  <Label class="text-[11px] font-black uppercase text-slate-400 tracking-widest">AI 供应商</Label>
                  <div class="flex p-1 bg-slate-100 dark:bg-slate-800 rounded-xl">
                    <button v-for="m in (['openai', 'ollama'] as const)" :key="m"
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

                  <div v-if="preferences.aiSupplier.supplier === 'openai'"
                    class="group flex items-center gap-3 px-4 py-3 bg-slate-50 dark:bg-slate-800/50 rounded-2xl border border-transparent focus-within:border-primary/30 focus-within:bg-white dark:focus-within:bg-slate-900 transition-all">
                    <KeyRound class="w-4 h-4 text-slate-400 group-focus-within:text-primary transition-colors" />
                    <input v-model="preferences.aiSupplier.apiKey" type="password" placeholder="OpenAI API Key"
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
                    <Label class="text-sm font-bold cursor-pointer text-slate-700 dark:text-slate-300">清理失效书签</Label>
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
                    class="w-12 text-center bg-transparent text-xs font-bold outline-none" />
                  <span class="text-[10px] font-bold text-slate-400 uppercase">ms</span>
                </div>
              </div>
            </div>
          </CardContent>

          <CardFooter
            class="px-10 py-8 bg-slate-50/50 dark:bg-slate-900/50 flex justify-between items-center border-t border-white dark:border-slate-800">
            <Button variant="ghost" size="sm" @click="reset"
              class="text-slate-400 hover:text-primary transition-colors">
              <RotateCcw class="w-4 h-4 mr-2" /> 恢复默认
            </Button>

            <div class="flex gap-3">
              <Button v-if="currentStep > 0" variant="ghost" @click="currentStep--" class="font-bold">返回</Button>
              <Button @click="currentStep === 2 ? saveAndFinish() : currentStep++" :disabled="!canGoNext"
                class="min-w-30 rounded-2xl font-black shadow-lg shadow-primary/20 hover:shadow-primary/40 transition-all active:scale-95">
                {{ currentStep === 2 ? '完成并开启' : '下一步' }}
                <ArrowRight class="ml-2 w-4 h-4" />
              </Button>
            </div>
          </CardFooter>
        </Card>
      </transition>

    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, reactive } from 'vue'
import StorageUtil from '@/utils/storageUtil'
import {
  Check, Bookmark, ShieldCheck, Zap,
  Loader2, Sparkles, Trash2, LayoutGrid, Globe, Clock,
  KeyRound, Network, Settings2, RotateCcw, ArrowRight
} from 'lucide-vue-next'
import Chrome from '@/assets/chrome.svg'
import Edge from '@/assets/edge.png'
import { Button } from '@/components/ui/button'
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from '@/components/ui/card'
import { Switch } from '@/components/ui/switch'
import { Label } from '@/components/ui/label'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog'
import { toast } from 'vue-sonner'
import { getAiModels, checkBrowsers, backupBookmarks } from '@/api'
import { Ebrowser } from '@/api'
defineExpose({ Ebrowser })

// --- 状态定义 ---
const currentStep = ref(0)                  // 当前步骤
const isAuthorizing = ref(false)            // 是否正在授权
const isVerifying = ref(false)              // 是否正在验证
const isVerified = ref(false)               // 是否验证通过
const availableModels = ref<string[]>([])   // 可用模型列表
const browsers = reactive({
  chrome: false,
  edge: false,
})                                          // 可用浏览器列表

const smartCategorizeClone = ref(StorageUtil.get<boolean>('smartCategorize') || true)
const automaticallyDeleteInvalidatedBookmarksClone = ref(StorageUtil.get<boolean>('automaticallyDeleteInvalidatedBookmarks') || false)
const systemProxyClone = ref(StorageUtil.get<boolean>('systemProxy') || true)


interface AiSupplier {
  supplier: 'openai' | 'ollama'
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
  apiKey: '',
  model: '',
}

const OllamaSupplier: AiSupplier = {
  supplier: 'ollama',
  apiUrl: 'http://localhost:11434/api',
  apiKey: import.meta.env.OPENAI_KEY || '',
  model: '',
}

const preferences = ref<Preferences>({
  smartCategorize: true,
  aiSupplier: OpenAISupplier,
  automaticallyDeleteInvalidatedBookmarks: true,
  timeout: 10000,
  systemProxy: true,
  userAuthorization: false,
  isInit: false,
})

// --- 核心逻辑 ---
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

const reset = () => {
  initLocalStorage()
  syncFromStorage()
}

onMounted(() => {
  syncFromStorage()
  checkBrowsers().then(res => {
    browsers.chrome = res.chrome
    browsers.edge = res.edge
  })
  if (!preferences.value.isInit) {
    reset()
  }
})

watch(() => preferences.value.aiSupplier.supplier, (newVal) => {
  isVerified.value = false
  preferences.value.aiSupplier.apiUrl = newVal === OpenAISupplier.supplier ? OpenAISupplier.apiUrl : OllamaSupplier.apiUrl
})

const handleBackupBookmarks = async (browser: Ebrowser) => {
  await backupBookmarks(browser).then((path) => {
    console.log(path)
    toast.success('书签备份成功，备份文件路径: ' + path)
  }).catch((err) => {
    toast.error('书签备份失败: ' + err)
  })
}



const handleAuthorize = async () => {
  isAuthorizing.value = true
  await new Promise(resolve => setTimeout(resolve, 1000))
  preferences.value.userAuthorization = true
  StorageUtil.set<boolean>('userAuthorization', true)
  isAuthorizing.value = false
}

const verifyAI = async () => {
  isVerifying.value = true
  console.log(preferences.value)
  availableModels.value = await getAiModels(preferences.value.aiSupplier.supplier, preferences.value.aiSupplier.apiUrl, preferences.value.aiSupplier.apiKey).then(res => {
    console.log(res)
    return res
  })
  isVerified.value = true
  isVerifying.value = false
  preferences.value.aiSupplier.model = availableModels.value[0]
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
}

// --- UI 数据 ---
const steps = [
  { title: '欢迎', icon: Bookmark, desc: '欢迎使用' },
  { title: '权限', icon: ShieldCheck, desc: '权限授予' },
  { title: '偏好', icon: Settings2, desc: '偏好设置' },
]
const progress = computed(() => (currentStep.value / (steps.length - 1)) * 100)
const canGoNext = computed(() => {
  if (currentStep.value === 1) return preferences.value.userAuthorization
  if (currentStep.value === 2) return isVerified.value && !!preferences.value.aiSupplier.model
  return true
})
</script>

<style scoped>
.card-slide-enter-active,
.card-slide-leave-active {
  transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);
}

.card-slide-enter-from {
  opacity: 0;
  transform: translateX(30px) scale(0.98);
}

.card-slide-leave-to {
  opacity: 0;
  transform: translateX(-30px) scale(0.98);
}

input::-webkit-outer-spin-button,
input::-webkit-inner-spin-button {
  -webkit-appearance: none;
  margin: 0;
}
</style>