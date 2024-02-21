import { buttonStyles } from '@gear-js/ui';
import clsx from 'clsx';
import { Link } from 'react-router-dom';
import { MenuHandler, Header as CommonHeader } from '@dapps-frontend/ui';
import logo from 'assets/images/logo.png';
import { useFTBalance } from 'hooks/api';
import styles from './Header.module.scss';

function Header() {
  const tokens = useFTBalance();

  return (
    <CommonHeader
      logo={
        <Link to="/">
          <img src={logo} alt="logo" className={styles.logo} />
        </Link>
      }
      menu={
        <MenuHandler
          className={{
            mobileMenuWrapper: styles.mobileMenuWrapper,
            icon: styles.menuIcon,
            menuOptions: {
              item: styles.menuOptionsItem,
              nativeIcon: styles.menuOptionsIcon,
            },
            wallet: {
              balance: styles.balance,
            },
          }}
        />
      }
      className={{
        header: styles.header,
        content: styles.content,
      }}>
      <>
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
      </>
    </CommonHeader>
  );
}

export { Header };
