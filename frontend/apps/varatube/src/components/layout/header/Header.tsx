import { buttonStyles } from '@gear-js/ui';
import clsx from 'clsx';
import { Link } from 'react-router-dom';
import { MenuHandler } from '@dapps-frontend/ui';
import logo from 'assets/images/logo.png';
import { useFTBalance } from 'hooks/api';
import styles from './Header.module.scss';

function Header() {
  const tokens = useFTBalance();

  return (
    <header className={styles.header}>
      <div className={styles.content}>
        <Link to="/">
          <img src={logo} alt="logo" className={styles.logo} />
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

        <MenuHandler
          className={{
            mobileMenuWrapper: styles.mobileMenuWrapper,
            menuOptions: {
              item: styles.menuOptionsItem,
              nativeIcon: styles.menuOptionsIcon,
            },
          }}
        />
      </div>
    </header>
  );
}

export { Header };
