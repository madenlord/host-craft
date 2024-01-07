import { invoke } from '@tauri-apps/api/tauri';

export async function runServer() {
    try {
        await invoke('run_server');
    } catch(err) {
        console.log(err);
    }
}

export async function stopServer() {
    try {
        await invoke('stop_server');
    } catch(err) {
        console.log(err);
    }
}