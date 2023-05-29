import { ReactComponent as VaraLogoSVG } from 'assets/images/vara-logo.svg';
import { Socials } from './socials';
import { Copyright } from './copyright';
import styles from './Footer.module.scss';

function Footer() {
  return (
    <footer className={styles.footer}>
      <VaraLogoSVG />
      <Copyright />
      <Socials />
    </footer>
  );
}

export { Footer };
