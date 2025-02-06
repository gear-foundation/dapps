import { useAccount } from '@gear-js/react-hooks';
import { useLocation } from 'react-router-dom';

import { MenuHandler, Header as CommonHeader } from '@dapps-frontend/ui';

import logo from '@/assets/icons/logo.svg';
import { useMediaQuery } from '@/hooks';
import { Link } from '@/ui';
import { cx } from '@/utils';

import { HeaderProps } from './Header.interfaces';
import styles from './Header.module.scss';

function Header({ menu }: HeaderProps) {
  const location = useLocation();
  const { account } = useAccount();
  const isMobile = useMediaQuery(600);

  return (
    <CommonHeader
      logo={
        <Link to="/">
          <img src={logo} alt="" />
        </Link>
      }
      menu={<MenuHandler />}
      className={{
        header: styles.header,
        content: styles.content,
      }}>
      {account && !isMobile && (
        <nav className={cx(styles.menu)}>
          {Object.keys(menu).map((item) => {
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
    </CommonHeader>
  );
}

export { Header };
