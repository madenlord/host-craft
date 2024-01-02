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