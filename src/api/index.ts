import { invoke } from "@tauri-apps/api/core"
import { listen, UnlistenFn } from '@tauri-apps/api/event'

export interface IBrowsers {
    chrome: boolean,
    edge: boolean,
}

export enum Ebrowser {
    Chrome = 'chrome',
    Edge = 'edge',
}


export interface IBookmarkFolder {
    id: number,
    name: string,
    children: IBookmarkFolder[],
}

export interface IBookmark {
    id: number,
    name: string,
    url: string,
    web_title: string,
    description: string,
    keywords: string,
    status: string,
    type: string,
    children: IBookmarkFolder[],
}

export const getAiModels = async (provider: string, apiUrl: string, apiKey: string) => {
    const res: string[] = await invoke('get_ai_models', {
        provider,
        apiUrl,
        apiKey,
    })
    return res
}

export const checkBrowsers = async () => {
    const res: IBrowsers = await invoke('check_browsers', {})
    return res
}

export const backupBookmarks = async (browser: Ebrowser) => {
    return await invoke('backup_bookmarks', { browser })
}

export const checkBackup = async (browser: Ebrowser) => {
    const res: boolean = await invoke('check_backup', { browser })
    return res
}

export const getBookmarkFolders = async (browser: Ebrowser) => {
    const res: IBookmarkFolder[] = await invoke('get_bookmark_folders', { browser })
    return res
}

export const getBookmarksByFolder = async (browser: Ebrowser, folderId: number) => {
    const res: IBookmark[] = await invoke('get_bookmarks_by_folder', { browser, folderId })
    return res
}

export const getBookmarksNum = async (browser: Ebrowser) => {
    const res: number = await invoke('get_bookmarks_num', { browser })
    return res
}

export interface IClassificationTask {
    title: string,
    url: string,
    description?: string,
    keywords?: string,
    taxonomy: IBookmarkFolder[],
}

export interface IClassifyOptions {
    auto_sort_local: boolean,
    auto_sort_dead: boolean,
    provider: string,
    api_key?: string,
    model: string,
    auto_delete: boolean,
    system_prompt?: string,
    system_proxy: boolean,
    timeout_secs?: number,
    max_tasks?: number,
}

export const smartClassifyV3 = async (task: IClassificationTask, options: IClassifyOptions) => {
    const res: IBookmarkFolder = await invoke('smart_classify_v3', { task, options })
    return res
}

export const getAllBookmarks = async (browser: Ebrowser) => {
    const res: IBookmark[] = await invoke('get_all_bookmarks', { browser })
    return res
}

export const organizeBookmarks = async (browser: Ebrowser, task: IClassificationTask, options: IClassifyOptions) => {
    const res: IBookmarkFolder[] = await invoke('organize_bookmarks', { browser, task, options })
    return res
}

export interface IProgressPayload {
    current: number;
    total: number;
    message: string;
}

export const listenToOrganizeProgress = async (
    onProgress: (payload: IProgressPayload) => void
): Promise<UnlistenFn> => {
    const unlisten = await listen<IProgressPayload>(
        'organize-progress',
        (event) => {
            onProgress(event.payload);
        }
    );

    return unlisten;
}