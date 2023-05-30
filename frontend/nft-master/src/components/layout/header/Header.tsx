import { Wallet } from 'features/wallet';
import { NodeSwitch } from 'features/node-switch';
import { Logo } from './logo';
import styles from './Header.module.scss';

function Header() {
  return (
    <header className={styles.header}>
      <Logo />

      <div className={styles.config}>
        <NodeSwitch />
        <Wallet />
      </div>
    </header>
  );
}

export { Header };
