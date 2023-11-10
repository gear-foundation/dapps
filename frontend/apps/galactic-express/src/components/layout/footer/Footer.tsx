import { Socials } from './socials';
import { Copyright } from './copyright';
import { ExplorerLink } from '../explorer-link';
import styles from './Footer.module.scss';

function Footer() {
  return (
    <footer className={styles.footer}>
      <div className={styles['socials-wrapper']}>
        <Socials />
        <Copyright />
      </div>
      <ExplorerLink />
    </footer>
  );
}

export { Footer };
