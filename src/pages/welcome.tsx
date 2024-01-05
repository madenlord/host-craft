import { Form, redirect } from "react-router-dom";

import FormInput from "../components/FormInput";
import SubmitButton from "../components/SubmitButton";
import { initializeRepo } from "../services/repoConfig";

import styles from "../style/shared.module.css";

interface ActionData {
  request: Request;
}

export async function action({ request }: ActionData) {
  const formData: FormData = await request.formData();
  const configuration = Object.fromEntries(formData);
  await initializeRepo(configuration);

  return redirect("/home");
}

export default function Welcome() {
  return (
    <>
      <header className={styles.welcomeHeader}>
        <h1>Welcome to HostCraft!</h1>
      </header>
      <div className={styles.wide}>
        <p>
          Please enter your user data (username and email) and the URL where the
          world data will be uploaded.
        </p>
        <Form method="post">
          <FormInput label="User name:" type="text" inputName="username" />
          <FormInput label="Repository URL:" type="text" inputName="url" />
          <div className={`${styles.buttonColumn} ${styles.footer}`}>
            <SubmitButton text="Save" />
          </div>
        </Form>
      </div>
    </>
  );
}
