import { useAccount } from '@gear-js/react-hooks';

import { Wallet } from '@dapps-frontend/ui';

import styles from './Header.module.scss';
import { CreateLink } from './create-link';
import { Logo } from './logo';

function Header() {
  const { account } = useAccount();

  return (
    <header className={styles.header}>
      <nav className={styles.nav}>
        <Logo />
        {account && <CreateLink />}
      </nav>

      <Wallet theme="gear" />
    </header>
  );
}

export { Header };
