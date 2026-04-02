<template>
    <div
        class="p-8 h-screen w-full flex flex-col gap-8 bg-slate-50/30 dark:bg-slate-950 overflow-hidden animate-in slide-in-from-bottom-6 duration-500">
        <header class="flex justify-between items-end shrink-0 gap-4">
            <div>
                <h1 class="text-4xl font-black text-slate-800 dark:text-slate-100 tracking-tight">分类管理</h1>
                <p class="text-slate-500 mt-2 text-sm font-medium">定义 AI 整理时使用的分类体系</p>
            </div>
            <div class="flex-1"></div>
            <Button variant="outline" class="rounded-2xl h-12 px-6 font-bold shadow-xl shadow-primary/20"
                @click="importCategory()">
                <Upload class="w-4 h-4 mr-2" /> 导入
            </Button>
            <Button variant="outline" class="rounded-2xl h-12 px-6 font-bold shadow-xl shadow-primary/20"
                @click="exportCategory(systems[0])">
                <Download class="w-4 h-4 mr-2" /> 导出默认分类
            </Button>
            <Button class="rounded-2xl h-12 px-6 font-bold shadow-xl shadow-primary/20"
                @click="sysDialogVisible = true">
                <Plus class="w-4 h-4 mr-2" /> 新建分类体系
            </Button>
        </header>

        <div
            class="flex-1 bg-white/80 dark:bg-slate-900/80 backdrop-blur-xl rounded-4xl border border-slate-100 dark:border-slate-800 shadow-2xl shadow-slate-200/50 dark:shadow-none overflow-hidden flex flex-col">
            <Tabs v-model="activeSystemId" class="flex-1 flex flex-col h-full w-full">

                <TabsList class="flex w-full justify-start p-0
               border-b border-slate-100 dark:border-slate-800
               bg-slate-50/50 dark:bg-slate-900/50">

                    <TabsTrigger v-for="sys in systems" :key="sys.id" :value="sys.id" class="group flex items-center gap-2 h-10 rounded-none
                   text-sm font-semibold
                   text-slate-500
                   data-[state=active]:text-slate-900 dark:data-[state=active]:text-white
                   data-[state=active]:bg-white dark:data-[state=active]:bg-slate-900
                   border border-transparent
                   data-[state=active]:border-slate-200 dark:data-[state=active]:border-slate-700
                   data-[state=active]:border-b-transparent
                   transition-all">

                        <ShieldCheck v-if="sys.isDefault" class="w-4 h-4 text-emerald-500" />
                        <UserCog v-else class="w-4 h-4 text-purple-500" />

                        <span class="truncate max-w-30">
                            {{ sys.name }}
                        </span>

                        <div v-if="!sys.isDefault" @click.stop="confirmDeleteSystem(sys.id)" class="ml-1 p-1 rounded-md opacity-0 group-hover:opacity-100
                       hover:bg-red-100 dark:hover:bg-red-900/30 transition">
                            <X class="w-3 h-3 text-slate-400 hover:text-red-500" />
                        </div>
                    </TabsTrigger>

                </TabsList>

                <TabsContent v-for="sys in systems" :key="sys.id" :value="sys.id"
                    class="flex-1 p-6 m-0 outline-none h-full">
                    <div class="h-[calc(100vh-280px)] overflow-y-auto pr-4">
                        <ElTree :data="sys.children" :props="defaultProps" :default-expand-all="true"
                            :expand-on-click-node="false" class="custom-el-tree bg-transparent" empty-text="暂无分类数据">
                            <template #default="{ node, data }">
                                <div
                                    class="flex-1 flex items-center justify-between py-2 pr-4 group rounded-xl hover:bg-slate-50 dark:hover:bg-slate-800/50 transition-colors w-full">
                                    <div class="flex items-center gap-2 truncate pl-2">
                                        <Folder v-if="sys.isDefault"
                                            class="w-4 h-4 text-emerald-400 fill-emerald-400/20" />
                                        <Folder v-else class="w-4 h-4 text-amber-400 fill-amber-400/20" />
                                        <span class="text-sm font-bold truncate text-slate-700 dark:text-slate-300">
                                            {{ node.label }}
                                        </span>
                                    </div>

                                    <div v-if="!sys.isDefault"
                                        class="opacity-0 group-hover:opacity-100 transition-opacity flex gap-1">
                                        <Button variant="ghost" size="icon"
                                            class="w-8 h-8 rounded-lg hover:text-blue-500 hover:bg-blue-50 dark:hover:bg-blue-900/30"
                                            @click.stop="openNodeDialog('add', sys, data)" title="添加子分类">
                                            <FolderPlus class="w-4 h-4" />
                                        </Button>
                                        <Button variant="ghost" size="icon"
                                            class="w-8 h-8 rounded-lg hover:text-amber-500 hover:bg-amber-50 dark:hover:bg-amber-900/30"
                                            @click.stop="openNodeDialog('edit', sys, data)" title="重命名">
                                            <Edit2 class="w-4 h-4" />
                                        </Button>
                                        <Button v-if="data.id !== 1 && data.id !== 997 && data.id !== 998 && data.id !== 999" variant="ghost" size="icon"
                                            class="w-8 h-8 rounded-lg hover:text-red-500 hover:bg-red-50 dark:hover:bg-red-900/30"
                                            @click.stop="confirmDeleteNode(sys, node, data)" title="删除">
                                            <Trash2 class="w-4 h-4" />
                                        </Button>
                                    </div>
                                    <div v-else class="pr-2">
                                        <Lock class="w-3.5 h-3.5 text-slate-300 dark:text-slate-600" />
                                    </div>
                                </div>
                            </template>
                        </ElTree>
                    </div>
                </TabsContent>
            </Tabs>
        </div>

        <Dialog v-model:open="sysDialogVisible">
            <DialogContent class="sm:max-w-106.25 rounded-3xl!">
                <DialogHeader>
                    <DialogTitle>新建分类体系</DialogTitle>
                </DialogHeader>
                <div class="grid gap-4 py-4">
                    <p class="text-sm text-slate-500">请输入新分类体系的名称：</p>
                    <Input v-model="newSystemName" placeholder="例如：我的工作书签" maxlength="20"
                        @keyup.enter="createNewSystem" />
                </div>
                <DialogFooter>
                    <Button variant="outline" @click="sysDialogVisible = false" class="rounded-xl">取消</Button>
                    <Button @click="createNewSystem" :disabled="!newSystemName.trim()" class="rounded-xl">确认创建</Button>
                </DialogFooter>
            </DialogContent>
        </Dialog>

        <Dialog v-model:open="nodeDialogVisible">
            <DialogContent class="sm:max-w-106.25 rounded-3xl!">
                <DialogHeader>
                    <DialogTitle>{{ nodeDialogType === 'add' ? '添加子分类' : '重命名分类' }}</DialogTitle>
                </DialogHeader>
                <div class="grid gap-4 py-4">
                    <Input v-model="nodeInputValue" :placeholder="nodeDialogType === 'add' ? '请输入新子分类名称' : '请输入新名称'"
                        @keyup.enter="handleNodeAction" />
                </div>
                <DialogFooter>
                    <Button variant="outline" @click="nodeDialogVisible = false" class="rounded-xl">取消</Button>
                    <Button @click="handleNodeAction" :disabled="!nodeInputValue.trim()" class="rounded-xl">确认</Button>
                </DialogFooter>
            </DialogContent>
        </Dialog>

        <AlertDialog v-model:open="alertVisible">
            <AlertDialogContent class="rounded-3xl!">
                <AlertDialogHeader>
                    <AlertDialogTitle>确认删除？</AlertDialogTitle>
                    <AlertDialogDescription>
                        {{ alertType === 'system' ? '确定要删除整个分类体系吗？此操作不可恢复。' : '确定删除该分类及其下属所有子分类吗？此操作不可恢复。' }}
                    </AlertDialogDescription>
                </AlertDialogHeader>
                <AlertDialogFooter>
                    <AlertDialogCancel @click="alertVisible = false" class="rounded-xl">取消</AlertDialogCancel>
                    <AlertDialogAction @click="handleDeleteConfirm"
                        class="rounded-xl bg-red-500 hover:bg-red-600 text-white">
                        确认删除
                    </AlertDialogAction>
                </AlertDialogFooter>
            </AlertDialogContent>
        </AlertDialog>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { ElTree } from 'element-plus'
import {
    Plus, ShieldCheck, UserCog, Folder, Lock, FolderPlus, Edit2, Trash2, X, Upload, Download
} from 'lucide-vue-next'
import StorageUtil from '@/utils/storageUtil'
import type { IBookmarkFolder } from '@/api'
import classification from '@/classification.json?raw'
import { readTextFile, writeTextFile } from '@tauri-apps/plugin-fs'
import { open, save } from '@tauri-apps/plugin-dialog'

import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Tabs, TabsList, TabsTrigger, TabsContent } from '@/components/ui/tabs'
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogFooter } from '@/components/ui/dialog'
import { AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle } from '@/components/ui/alert-dialog'
import { toast } from 'vue-sonner'

// --- 数据结构定义 ---
interface CategorySystem {
    id: string,
    name: string,
    isDefault: boolean,
    children: IBookmarkFolder[];
}

// --- 状态与常量 ---
const STORAGE_PREFIX = 'CATEGORY_SYS_'
const activeSystemId = ref('default')
const systems = ref<CategorySystem[]>([])
const defaultProps = { children: 'children', label: 'name' }

// 弹窗控制状态
const sysDialogVisible = ref(false)
const newSystemName = ref('')

const nodeDialogVisible = ref(false)
const nodeDialogType = ref<'add' | 'edit'>('add')
const nodeInputValue = ref('')
const currentActionSys = ref<CategorySystem | null>(null)
const currentActionNodeData = ref<IBookmarkFolder | null>(null)

const alertVisible = ref(false)
const alertType = ref<'system' | 'node'>('system')
const alertTargetId = ref<string>('')
const alertNodeContext = ref<any>(null) 

onMounted(() => {
    loadSystems()
})

const loadSystems = () => {
    const defaultSystem: CategorySystem = {
        id: 'default',
        name: 'Default',
        isDefault: true,
        children: JSON.parse(classification)
    }

    const loadedSystems: CategorySystem[] = [defaultSystem]

    for (let i = 0; i < localStorage.length; i++) {
        const key = localStorage.key(i)
        if (key && key.startsWith(STORAGE_PREFIX)) {
            const sysData = StorageUtil.get<CategorySystem>(key)
            if (sysData) loadedSystems.push(sysData)
        }
    }
    systems.value = loadedSystems
}

const saveCurrentSystem = (sys: CategorySystem | null) => {
    if (!sys || sys.isDefault) return
    StorageUtil.set(STORAGE_PREFIX + sys.id, sys)
}

const createNewSystem = () => {
    const name = newSystemName.value.trim()
    if (!name) return

    const sysId = `sys_${Date.now()}`
    const newSystem: CategorySystem = {
        id: sysId,
        name: name,
        isDefault: false,
        children: [
            { 
                id: 1, 
                name: '书签栏', 
                type: 'folder',
                children: [
                    { id: 997, name: '本地链接', type: 'folder', children: [] },
                    { id: 998, name: '失效链接', type: 'folder', children: [] },
                    { id: 999, name: '其他', type: 'folder', children: [] },
                ] 
            }
        ]
    }

    StorageUtil.set(STORAGE_PREFIX + sysId, newSystem)
    systems.value.push(newSystem)
    activeSystemId.value = sysId

    sysDialogVisible.value = false
    newSystemName.value = ''
}

const confirmDeleteSystem = (sysId: string) => {
    alertType.value = 'system'
    alertTargetId.value = sysId
    alertVisible.value = true
}

const importCategory = async () => {
    try {
        const path = await open({
            filters: [{ name: 'JSON Files', extensions: ['json'] }],
            multiple: false 
        });
        
        if (!path) return; 

        const json = await readTextFile(path as string);
        const importedData = JSON.parse(json);

        if (!importedData || typeof importedData !== 'object') {
            throw new Error('无效的分类数据格式');
        }

        let importedName = importedData.name || '导入的分类';
        let importedChildren = importedData.children || [];

        if (Array.isArray(importedData)) {
            importedChildren = importedData;
            importedName = '导入的分类体系';
        }

        const sysId = `sys_${Date.now()}`;
        const newSystem: CategorySystem = {
            id: sysId,
            name: `${importedName} (导入)`,
            isDefault: false, 
            children: importedChildren
        };

        StorageUtil.set(STORAGE_PREFIX + sysId, newSystem);
        systems.value.push(newSystem);
        activeSystemId.value = sysId; 

    } catch (error) {
        console.error('导入分类失败:', error);
        toast.error('导入分类失败，请检查文件格式是否正确');
    }
}

const exportCategory = async (category: CategorySystem) => {
    try {
        const json = JSON.stringify(category, null, 2);

        const path = await save({
            filters: [{ name: 'JSON Files', extensions: ['json'] }],
            defaultPath: `${category.name}.json` 
        });
        
        if (path) {
            await writeTextFile(path, json);
        }
    } catch (error) {
         toast.error('导出分类失败，请检查文件格式是否正确');
    }
}


// --- 节点操作逻辑 ---
const openNodeDialog = (type: 'add' | 'edit', sys: CategorySystem, data: IBookmarkFolder) => {
    nodeDialogType.value = type
    currentActionSys.value = sys
    currentActionNodeData.value = data
    nodeInputValue.value = type === 'edit' ? data.name : ''
    nodeDialogVisible.value = true
}

const handleNodeAction = () => {
    const val = nodeInputValue.value.trim()
    if (!val || !currentActionSys.value || !currentActionNodeData.value) return

    if (nodeDialogType.value === 'add') {
        const parentData = currentActionNodeData.value
        if (!parentData.children) {
            parentData.children = []
        }

        let newId: number

        if (parentData.id === 1) {
            const childrenIds = parentData.children
                .map(c => Number(c.id))
                .filter(id => !isNaN(id) && id < 997)

            newId = childrenIds.length === 0 ? 100 : Math.max(...childrenIds) + 1

            if (newId > 996) {
                toast.error('父级分类数量已达上限（最多创建至996）')
                return
            }

            const newNode = { id: newId, name: val, type: 'folder', children: [] }

            const insertIndex = parentData.children.findIndex(c => Number(c.id) >= 997)
            if (insertIndex !== -1) {
                parentData.children.splice(insertIndex, 0, newNode)
            } else {
                parentData.children.push(newNode)
            }

        } else {
            const parentLen = String(parentData.id).length
            
            if (parentLen >= 7) {
                toast.error('最多只能创建4级子分类')
                return
            }

            const expectedLen = parentLen + 1
            const startId = Math.pow(10, expectedLen - 1) 

            const childrenIds = parentData.children
                .map(c => Number(c.id))
                .filter(id => !isNaN(id))

            newId = childrenIds.length === 0 ? startId : Math.max(...childrenIds) + 1

            parentData.children.push({ id: newId, name: val, type: 'folder', children: [] })
        }
    } else {
        currentActionNodeData.value.name = val
    }

    saveCurrentSystem(currentActionSys.value)
    nodeDialogVisible.value = false
}

const confirmDeleteNode = (sys: CategorySystem, node: any, data: IBookmarkFolder) => {
    alertType.value = 'node'
    currentActionSys.value = sys
    currentActionNodeData.value = data
    alertNodeContext.value = node
    alertVisible.value = true
}

// --- 统一处理删除确认 ---
const handleDeleteConfirm = () => {
    if (alertType.value === 'system') {
        StorageUtil.remove(STORAGE_PREFIX + alertTargetId.value)
        systems.value = systems.value.filter(sys => sys.id !== alertTargetId.value)
        if (activeSystemId.value === alertTargetId.value) {
            activeSystemId.value = 'default'
        }
    } else if (alertType.value === 'node') {
        const parent = alertNodeContext.value.parent
        const children: IBookmarkFolder[] = parent.data.children || parent.data
        const index = children.findIndex(d => d.id === currentActionNodeData.value?.id)
        if (index > -1) {
            children.splice(index, 1)
            saveCurrentSystem(currentActionSys.value)
        }
    }
    alertVisible.value = false
}
</script>

<style scoped>
:deep(.custom-el-tree .el-tree-node__content) {
    height: auto;
    background-color: transparent !important;
}

:deep(.custom-el-tree .el-tree-node__content:hover) {
    background-color: transparent !important;
}

:deep(.custom-el-tree .el-tree-node:focus > .el-tree-node__content) {
    background-color: transparent !important;
}
</style>