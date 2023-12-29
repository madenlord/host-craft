import LinkButton from '../components/LinkButton';
import Title from '../components/Title';

import styles from '../style/shared.module.css';

export default function Index() {
    return (
    <>
      <Title />
      <div className={`${styles.buttonColumn} ${styles.footer}`}>
        <LinkButton to='settings' text='Settings' />
        <LinkButton to='run' text='Run' />
      </div>
    </>
  );
}