import { invoke } from '@tauri-apps/api/tauri';

export type ServerConfig  = {
    mem_max: string;
    mem_init: string;
    gui: boolean;    
}

export async function getServerConfig() {
    let serverConfig = {}

    try {
        const serverConfigJson: string = await invoke('get_server_config');
        serverConfig = JSON.parse(serverConfigJson);
    } catch(err) {
        console.log(err);
    }

    return serverConfig;
}