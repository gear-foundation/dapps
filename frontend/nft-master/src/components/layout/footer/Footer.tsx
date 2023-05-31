import { ReactComponent as VaraLogoSVG } from 'assets/images/vara-logo.svg';
import { Socials } from './socials';
import { Copyright } from './copyright';
import styles from './Footer.module.scss';
import { Container } from '../container';

function Footer() {
  return (
    <footer>
      <Container className={styles.container}>
        <VaraLogoSVG />
        <Copyright />
        <Socials />
      </Container>
    </footer>
  );
}

export { Footer };
