import { Wallet } from '@/features/wallet';

import styles from './Header.module.scss';
import { Logo } from './logo';

function Header() {
  return (
    <header className={styles.header}>
      <Logo />
      <Wallet />
    </header>
  );
}

export { Header };
