import { invoke } from '@tauri-apps/api/tauri';

export type RepoConfig = {
    username: string;
    url: string
}

export async function isRepoInitialized() {
    return await invoke('is_repo_initialized');
}

export async function initializeRepo(configuration: any) {
    let repoConfig = {}

    try {
        Object.assign(repoConfig, configuration);
        await(invoke(
            'init_repo',
            { repo_config: JSON.stringify(repoConfig) }
        )); 
    } catch(err) {
        console.log(err);
    }
}

export async function getRepoConfig() {
    let repoConfig = {}

    try {
        const repoConfigJson: string = await invoke('get_repo_config');
        repoConfig = JSON.parse(repoConfigJson);
    } catch(err) {
        console.log(err);
    }

    return repoConfig;
}

export async function updateRepoConfig(updates: any) {
    let repoConfig = {};

    try {
        Object.assign(repoConfig, updates);
        await invoke(
            'update_repo_config',
            { json_config: JSON.stringify(repoConfig) }
        );
    } catch(err) {
        console.log(err);
    }
}