import { Link } from 'react-router-dom';
import { Wallet } from '@dapps-frontend/ui';
import { ReactComponent as LogoSVG } from 'assets/images/logo.svg';
import styles from './Header.module.scss';

function Header() {
  return (
    <header className={styles.header}>
      <Link to="/">
        <LogoSVG />
      </Link>

      <Wallet />
    </header>
  );
}

export { Header };
