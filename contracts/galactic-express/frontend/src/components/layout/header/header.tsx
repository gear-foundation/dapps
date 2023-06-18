import { NavLink } from 'react-router-dom';
import { Wallet } from 'features/wallet';
import { ROUTES } from 'consts';
import { Logo } from './logo';
import styles from './Header.module.scss';

function Header() {
  return (
    <header className={styles.header}>
      <nav className={styles.nav}>
        <Logo />

        <ul className={styles.menu}>
          <li>
            <NavLink to={ROUTES.HOME} className={styles.link}>
              Home
            </NavLink>
          </li>
          <li>
            <NavLink to={ROUTES.LEADERBOARD} className={styles.link}>
              Leaderboard
            </NavLink>
          </li>
        </ul>
      </nav>

      <Wallet />
    </header>
  );
}

export { Header };
