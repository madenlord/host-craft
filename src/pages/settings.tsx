import LinkButton from "../components/LinkButton";

import styles from "../style/shared.module.css";

export default function Settings() {
  return (
    <>
      <div className={`${styles.buttonColumn} ${styles.centeredNav}`}>
        <LinkButton to="server" text="Server" />
      </div>
      <div className={`${styles.buttonColumn} ${styles.footer}`}>
        <LinkButton to=".." text="Done" />
      </div>
    </>
  );
}
