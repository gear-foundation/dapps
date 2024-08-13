import { OnLogin } from 'components';
import { Wallet } from '@dapps-frontend/ui';
import { Logo } from './logo';
import { Menu } from './menu';
import styles from './Header.module.scss';

function Header() {
  return (
    <header className={styles.header}>
      <nav className={styles.nav}>
        <Logo />
        <OnLogin>
          <Menu />
        </OnLogin>
      </nav>

      <Wallet />
    </header>
  );
}

export { Header };
