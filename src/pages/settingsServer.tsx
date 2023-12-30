import { useState } from 'react';
import { Form, useLoaderData} from 'react-router-dom';

import FormField from '../components/FormField';
import SubmitButton from '../components/SubmitButton';
import { ServerConfig, getServerConfig } from '../services/serverConfig';

import styles from '../style/shared.module.css';

interface LoaderData {
    server: ServerConfig;
}

export async function loader() {
    const server = await getServerConfig();
    return { server };
}

export default function SettingsServer() {
    const [valid, setValid] = useState(true);
    const { server } = useLoaderData() as LoaderData;

    function handleValidityChange() {
      setValid(!valid);
    }
  
    return (
      <>
        <header className={styles.emptyHeader}></header>
        <div className={styles.wide}>
          <Form method="post">
              <FormField
                label="Maximum RAM memory:"
                type="text"
                inputName="mem_max"
                defaultValue={server.mem_max}
                validate={true}
                onValidityChange={handleValidityChange}
              />
              <FormField
                label="Memory at startup:"
                type="text"
                inputName="mem_init"
                defaultValue={server.mem_init}
                validate={true}
                onValidityChange={handleValidityChange}
              />
              <FormField
                label="Enable GUI:"
                type="checkbox"
                inputName="gui"
                defaultValue={server.gui}
              />
            <div className={`${styles.buttonColumn} ${styles.footer}`}>
              <SubmitButton text="Save" disabled={!valid} />
            </div>
          </Form>
        </div>
      </>
    );
  }