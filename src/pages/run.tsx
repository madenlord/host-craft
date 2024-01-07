import { useEffect } from 'react';

import LinkButton from '../components/LinkButton';
import { runServer, stopServer } from '../services/serverAPI';

import styles from '../style/shared.module.css';



export default function ServerExecution() {
    useEffect( () => {
        runServer();
    })

    function handleStop() {
        stopServer();
    }

    return (
        <>
            <header className={styles.emptyHeader}></header>
            <div className={styles.wide}>
                <p>Server is now running!</p>
            </div>
            <div className={`${styles.buttonColumn} ${styles.footer}`}>
                <LinkButton to='/home' text='Stop' onClick={handleStop} />
            </div>
        </>
    );
}