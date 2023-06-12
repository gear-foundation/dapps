import { Wallet } from 'features/wallet';
import { Logo } from './logo';
import styles from './Header.module.scss';

function Header() {
  return (
    <header className={styles.header}>
      <Logo />
      <Wallet />
    </header>
  );
}

export { Header };
