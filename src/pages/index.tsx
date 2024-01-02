import { useLoaderData } from 'react-router';

import Title from '../components/Title';
import ConfigDisplay from '../components/ConfigDisplay';
import LinkButton from '../components/LinkButton';
import {
    ServerConfig,
    getServerConfig
} from '../services/serverConfig';

import styles from '../style/shared.module.css';

interface LoaderData {
    server: ServerConfig;
}

export async function loader() {
    const server = await getServerConfig();

    return { server };
}

export default function Index() {
    const { server } = useLoaderData() as LoaderData;

    return (
    <>
      <Title />
      <ConfigDisplay server={server} />
      <div className={`${styles.buttonColumn} ${styles.footer}`}>
        <LinkButton to='settings' text='Settings' />
        <LinkButton to='run' text='Run' />
      </div>
    </>
  );
}