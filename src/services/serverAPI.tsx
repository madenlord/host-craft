import { invoke } from '@tauri-apps/api/tauri';

export async function runServer() {
    try {
        await invoke('run_server');
    } catch(err) {
        console.log(err);
    }
}

export async function getPublicIP(): Promise<string> {
    let publicIP: string = '';

    try {
        const response = await fetch('https://api.ipify.org');
        publicIP = await response.text();
    } catch(err) {
        console.log(err);
    }

    return publicIP;
}

export async function stopServer() {
    try {
        await invoke('stop_server');
    } catch(err) {
        console.log(err);
    }
}