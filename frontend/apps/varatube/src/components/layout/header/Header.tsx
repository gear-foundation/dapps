import { buttonStyles } from '@gear-js/ui';
import clsx from 'clsx';
import { Link } from 'react-router-dom';
import { Wallet } from '@dapps-frontend/ui';
import logo from 'assets/images/logo.png';
import { useFTBalance } from 'hooks/api';
import styles from './Header.module.scss';

function Header() {
  const tokens = useFTBalance();

  return (
    <header className={styles.header}>
      <Link to="/">
        <img src={logo} alt="" style={{ maxWidth: '150px' }} />
      </Link>

      <Link
        to="subscription"
        className={clsx(buttonStyles.button, buttonStyles.medium, buttonStyles.secondary, styles.link)}>
        My Subscription
      </Link>

      {tokens && (
        <p className={styles.balance}>
          <span className={styles.currency}>Tokens:</span> {tokens}
        </p>
      )}

      <Wallet />
    </header>
  );
}

export { Header };
