import { useAccount } from '@gear-js/react-hooks';
import { Wallet } from '@dapps-frontend/ui';
import { Logo } from './logo';
import { CreateLink } from './create-link';
import styles from './Header.module.scss';

function Header() {
  const { account } = useAccount();

  return (
    <header className={styles.header}>
      <nav className={styles.nav}>
        <Logo />
        {account && <CreateLink />}
      </nav>

      <Wallet />
    </header>
  );
}

export { Header };
