import { useAccount } from '@gear-js/react-hooks';
import { useLocation, Link } from 'react-router-dom';
import { Header as CommonHeader, MenuHandler } from '@dapps-frontend/ui';
import { ReactComponent as GalexSVG } from 'assets/images/logo.svg';
import { ReactComponent as VaraSVG } from 'assets/images/logo-vara.svg';
import { cx } from 'utils';
import styles from './Header.module.scss';

function Header() {
  const location = useLocation();
  const { account } = useAccount();

  return (
    <CommonHeader
      logo={
        <Link to="/">
          <VaraSVG className={cx(styles['vara-logo'])} />
          <GalexSVG />
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
      className={{ header: styles.header, content: styles.container }}
    />
  );
}

export { Header };
