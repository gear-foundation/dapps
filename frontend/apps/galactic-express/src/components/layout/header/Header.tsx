import { useAccount } from '@gear-js/react-hooks';
import { useLocation, Link } from 'react-router-dom';
import { Wallet } from '@dapps-frontend/ui';
import { ReactComponent as GalexSVG } from 'assets/images/logo.svg';
import { ReactComponent as VaraSVG } from 'assets/images/logo-vara.svg';
import { cx } from 'utils';
import styles from './Header.module.scss';

interface HeaderProps {
  menu?: Record<string, { url: string }>;
}

function Header({ menu }: HeaderProps) {
  const location = useLocation();
  const { account } = useAccount();

  return (
    <header className={cx(styles.header)}>
      <div className={cx(styles.container)}>
        <Link to="/">
          <VaraSVG className={cx(styles['vara-logo'])} />
          <GalexSVG />
        </Link>

        {account && (
          <nav className={cx(styles.menu)}>
            {menu &&
              Object.keys(menu).map((item) => {
                const { url } = menu[item];

                return (
                  <Link to={url} key={item}>
                    <p
                      className={cx(
                        styles['menu-item'],
                        location.pathname === `/${url}` ? styles['menu-item--active'] : '',
                      )}>
                      {item}
                    </p>
                  </Link>
                );
              })}
          </nav>
        )}

        <Wallet />
      </div>
    </header>
  );
}

export { Header };
