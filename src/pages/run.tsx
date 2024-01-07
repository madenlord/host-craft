import { useEffect } from 'react';
import { useLoaderData } from 'react-router-dom';

import LinkButton from '../components/LinkButton';
import { runServer, stopServer, getPublicIP } from '../services/serverAPI';

import styles from '../style/shared.module.css';

interface LoaderData {
    publicIP: string;
}

export async function loader() {
    const publicIP = await getPublicIP();
    return { publicIP };
}

export default function ServerExecution() {
    const { publicIP } = useLoaderData() as LoaderData;

    useEffect( () => {
        runServer();
    })

    function handleStop() {
        stopServer();
    }

    return (
        <>
            <header className={styles.emptyHeader}></header>
            <div className={`${styles.wide} ${styles.hCenter}`}>
                <p className="font-family: large">Server is now running!</p>
                <p className="font-family: large">Your public IP is: {publicIP}</p>
            </div>
            <div className={`${styles.buttonColumn} ${styles.footer}`}>
                <LinkButton to='/home' text='Stop' onClick={handleStop} />
            </div>
        </>
    );
}