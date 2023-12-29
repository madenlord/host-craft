import { invoke } from '@tauri-apps/api/tauri';

export type ServerConfig  = {
    mem_max: string;
    mem_init: string;
    gui: boolean;    
}

export async function getServerConfig() {

}