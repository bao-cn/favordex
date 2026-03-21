import { invoke } from "@tauri-apps/api/core"

export interface IBrowsers {
    chrome: boolean,
    edge: boolean,
}

export enum Ebrowser {
    Chrome = 'chrome',
    Edge = 'edge',
} 


export interface IBookmarkFolder {
    id: string,
    name: string,
    children: IBookmarkFolder[],
}

export interface IBookmark {
    name: string,
    url: string,
    web_title: string,
    description: string,
    keywords: string,
    status: string,
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

export const getBookmarkFolders = async (browser: Ebrowser) => {
    const res: IBookmarkFolder[] = await invoke('get_bookmark_folders', { browser })
    return res
}

export const getBookmarksByFolder = async (browser: Ebrowser, folderId: string) => {
    const res: IBookmark[] = await invoke('get_bookmarks_by_folder', { browser, folderId })
    return res
}

export const getBookmarksNum = async (browser: Ebrowser) => {
    const res: number = await invoke('get_bookmarks_num', { browser })
    return res
}
