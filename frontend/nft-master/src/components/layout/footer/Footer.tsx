import { Socials } from './socials';
import { Copyright } from './copyright';
import styles from './Footer.module.scss';
import { Container } from '../container';
import { VaraLogoIcon } from '../../../assets/images';

function Footer() {
  return (
    <footer>
      <Container className={styles.container}>
        <VaraLogoIcon />
        <Copyright />
        <Socials />
      </Container>
    </footer>
  );
}

export { Footer };
