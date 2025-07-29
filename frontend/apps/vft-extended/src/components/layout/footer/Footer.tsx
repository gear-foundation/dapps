import styles from './Footer.module.scss';
import { Copyright } from './copyright';
import { Socials } from './socials';

function Footer() {
  return (
    <footer className={styles.footer}>
      <Socials />
      <Copyright />
    </footer>
  );
}

export { Footer };
