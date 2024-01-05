import { useState } from 'react';
import { Form, useLoaderData, redirect } from 'react-router-dom';

import FormInput from '../components/FormInput';
import SubmitButton from '../components/SubmitButton';
import { 
    ServerConfig,
    getServerConfig,
    updateServerConfig
} from '../services/serverConfig';

import styles from '../style/shared.module.css';

interface LoaderData {
    server: ServerConfig;
}

interface ActionData {
    request: Request;
}

export async function loader() {
    const server = await getServerConfig();
    return { server };
}

export async function action({ request }: ActionData) {
    const formData: FormData = await request.formData();
    const updates = Object.fromEntries(formData);
    await updateServerConfig(updates);

    return redirect('/settings');
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
              <FormInput
                label="Maximum RAM memory:"
                type="text"
                inputName="mem_max"
                defaultValue={server.mem_max}
                validate={true}
                onValidityChange={handleValidityChange}
              />
              <FormInput
                label="Memory at startup:"
                type="text"
                inputName="mem_init"
                defaultValue={server.mem_init}
                validate={true}
                onValidityChange={handleValidityChange}
              />
              <FormInput
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