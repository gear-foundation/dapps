import { Logo } from './logo';
import styles from './Header.module.scss';



function Header() {
  return (
    <header className={styles.header}>
      <Logo />
    </header>
  );
}

export { Header };
