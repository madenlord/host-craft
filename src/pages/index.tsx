import { redirect } from 'react-router';

import { isRepoInitialized } from '../services/repoConfigAPI';

export async function loader() {
    let path: string;
    const initialized = await isRepoInitialized();
    initialized ? path = '/home' : path = '/welcome';
    
    return redirect(path);
}

export default function Index() {
    return (<></>);
}