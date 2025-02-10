import { Wallet } from '@dapps-frontend/ui';

import { OnLogin } from '@/components';

import styles from './Header.module.scss';
import { Logo } from './logo';
import { Menu } from './menu';

function Header() {
  return (
    <header className={styles.header}>
      <nav className={styles.nav}>
        <Logo />
        <OnLogin>
          <Menu />
        </OnLogin>
      </nav>

      <Wallet theme="gear" />
    </header>
  );
}

export { Header };
