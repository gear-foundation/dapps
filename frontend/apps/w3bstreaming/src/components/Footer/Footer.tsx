import { Container, Footer as VaraFooter } from '@dapps-frontend/ui';

import logo from '@/assets/icons/logo.svg';
import { cx } from '@/utils';

import styles from './Footer.module.scss';

function Footer() {
  return (
    <Container className={cx(styles.container)}>
      <img src={logo} alt="vara-logo" />
      <VaraFooter vara />
    </Container>
  );
}

export { Footer };
