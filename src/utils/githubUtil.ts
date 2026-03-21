export interface GitHubContributor {
    login: string; 
    id: number;
    node_id: string;
    avatar_url: string; 
    html_url: string; 
}

interface ContributorRequestConfig {
    owner: string;
    repo: string;
    token?: string;
    perPage?: number;
    page?: number;
}

export async function getGitHubContributors(
    config: ContributorRequestConfig
): Promise<GitHubContributor[]> {
    const {
        owner,
        repo,
        perPage = 30,
        page = 1
    } = config;

    const apiUrl = new URL(`https://api.github.com/repos/${owner}/${repo}/contributors`);
    apiUrl.searchParams.set('per_page', perPage.toString());
    apiUrl.searchParams.set('page', page.toString());

    const headers: HeadersInit = {
        'Accept': 'application/vnd.github.v3+json',
        'User-Agent': 'Favordex-GitHub-API-Client'
    };


    try {
        const response = await fetch(apiUrl.toString(), {
            method: 'GET',
            headers: headers,
            mode: 'cors'
        });

        if (!response.ok) {
            const errorData = await response.json().catch(() => ({ message: response.statusText }));
            throw new Error(`GitHub API 请求失败 [${response.status}]: ${errorData.message || '未知错误'}`);
        }

        const contributors = await response.json() as GitHubContributor[];
        return contributors;
    } catch (error) {
        if (error instanceof Error) {
            throw new Error(`获取贡献者失败: ${error.message}`);
        }
        throw new Error(`获取贡献者失败: 未知错误`);
    }
}