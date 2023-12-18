import { Container, Footer as VaraFooter } from '@dapps-frontend/ui';
import logo from '@/assets/icons/logo.svg';
import styles from './Footer.module.scss';
import { cx } from '@/utils';

function Footer() {
  return (
    <Container className={cx(styles.container)}>
      <img src={logo} alt="" />
      <VaraFooter vara />
    </Container>
  );
}

export { Footer };
