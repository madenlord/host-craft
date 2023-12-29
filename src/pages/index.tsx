import Title from '../components/Title';
import ConfigDisplay from '../components/ConfigDisplay';
import LinkButton from '../components/LinkButton';

import styles from '../style/shared.module.css';

export default function Index() {
    const serverDefault = {
        'mem_max': '2G',
        'mem_init': '512M',
        'gui': true
    }

    return (
    <>
      <Title />
      <ConfigDisplay server={serverDefault} />
      <div className={`${styles.buttonColumn} ${styles.footer}`}>
        <LinkButton to='settings' text='Settings' />
        <LinkButton to='run' text='Run' />
      </div>
    </>
  );
}