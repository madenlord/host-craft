import { useLoaderData } from 'react-router';

import Title from '../components/Title';
import ConfigDisplay from '../components/ConfigDisplay';
import LinkButton from '../components/LinkButton';
import {
    ServerConfig,
    getServerConfig
} from '../services/serverConfig';

import {
  RepoConfig,
  getRepoConfig
} from '../services/repoConfig';

import styles from '../style/shared.module.css';

interface LoaderData {
    server: ServerConfig;
    repo: RepoConfig;
}

export async function loader() {
    const server = await getServerConfig();
    const repo = await getRepoConfig();

    return { server, repo };
}

export default function Index() {
    const { server, repo } = useLoaderData() as LoaderData;

    return (
    <>
      <Title />
      <ConfigDisplay server={server} repo={repo} />
      <div className={`${styles.buttonColumn} ${styles.footer}`}>
        <LinkButton to='settings' text='Settings' />
        <LinkButton to='run' text='Run' />
      </div>
    </>
  );
}