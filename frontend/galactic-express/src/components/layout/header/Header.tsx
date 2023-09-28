import { useState } from 'react';
import { WalletModal, WalletInfo } from 'features/wallet/components';
import { Link, useLocation } from 'react-router-dom';
import { useAccount } from '@gear-js/react-hooks';
import { cx } from 'utils';
import styles from './Header.module.scss';
import { Logo } from './logo';

interface Menu {
  [key: string]: {
    url: string;
  };
}
interface HeaderProps {
  menu?: Menu;
}

function Header({ menu }: HeaderProps) {
  const location = useLocation();
  const { account } = useAccount();
  const [isWalletModalOpen, setIsWalletModalOpen] = useState<boolean>(false);

  const handleCloseWalletModal = () => {
    setIsWalletModalOpen(false);
  };

  return (
    <>
      <header className={cx(styles.header)}>
        <div className={cx(styles.container)}>
          <Logo />
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
          <WalletInfo account={account} />
        </div>
      </header>

      {isWalletModalOpen && <WalletModal onClose={handleCloseWalletModal} />}
    </>
  );
}

export { Header };
