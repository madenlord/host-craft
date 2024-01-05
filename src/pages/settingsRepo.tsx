import { Form, useLoaderData, redirect } from 'react-router-dom';

import FormInput from '../components/FormInput';
import SubmitButton from '../components/SubmitButton';
import { RepoConfig, getRepoConfig, updateRepoConfig } from '../services/repoConfig';

import styles from '../style/shared.module.css';

interface LoaderData {
    repo: RepoConfig;
}

interface ActionData {
    request: Request;
}

export async function loader() {
    const repo = await getRepoConfig();
    return { repo };
}

export async function action({ request }: ActionData) {
    const formData: FormData = await request.formData();
    const updates = Object.fromEntries(formData);
    await updateRepoConfig(updates);

    return redirect('/settings');
}

export default function SettingsRepo() {
    const { repo } = useLoaderData() as LoaderData;

    return (
        <>
          <header className={styles.emptyHeader}></header>
          <div className={styles.wide}>
            <Form method="post">
                <FormInput
                  label="User name:"
                  type="text"
                  inputName="username"
                  defaultValue={repo.username}
                />
                <FormInput
                  label="Repository URL:"
                  type="text"
                  inputName="url"
                  defaultValue={repo.url}
                />
              <div className={`${styles.buttonColumn} ${styles.footer}`}>
                <SubmitButton text="Save" />
              </div>
            </Form>
          </div>
        </>
      );
}