import { Link } from 'react-router-dom';
import { Header as CommonHeader, MenuHandler } from '@dapps-frontend/ui';
import { ReactComponent as VaraSVG } from 'assets/images/icons/logo-vara.svg';
import styles from './Header.module.scss';

function Header() {
  return (
    <CommonHeader
      logo={
        <Link to="/">
          <VaraSVG className={styles['vara-logo']} />
        </Link>
      }
      menu={
        <MenuHandler
          className={{
            wallet: {
              balance: styles.walletBalance,
            },
            icon: styles.menuIcon,
          }}
        />
      }
      className={{ header: styles.header, content: styles.container }}></CommonHeader>
  );
}

export { Header };
