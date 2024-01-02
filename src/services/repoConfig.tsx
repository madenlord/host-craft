import { invoke } from '@tauri-apps/api/tauri';

export type RepoConfig = {
    username: string;
    url: string
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