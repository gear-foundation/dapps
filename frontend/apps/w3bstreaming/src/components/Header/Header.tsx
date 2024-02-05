import { useEffect, useState } from 'react';
import { useLocation } from 'react-router-dom';
import { Button, Link } from '@ui';
import { useAccount } from '@gear-js/react-hooks';
import { WalletModal, WalletInfo } from '@/features/Wallet/components';
import { cx } from '@/utils';
import styles from './Header.module.scss';
import logo from '@/assets/icons/logo.svg';
import { HeaderProps } from './Header.interfaces';
import { useMediaQuery } from '@/hooks';
import menuIcon from '@/assets/icons/burger-menu-icon.svg';
import { BurgerMenu } from '../BurgerMenu/BurgerMenu';

function Header({ menu }: HeaderProps) {
  const location = useLocation();
  const { account } = useAccount();
  const [isWalletModalOpen, setIsWalletModalOpen] = useState<boolean>(false);
  const isMobile = useMediaQuery(600);
  const [isMobileMenuOpen, setIsMobileMenuOpen] = useState<boolean>(false);

  const burgerMenuHandler = () => {
    setIsMobileMenuOpen(false);
  };

  useEffect(() => {
    if (isMobileMenuOpen && !isMobile) {
      burgerMenuHandler();
    }
  }, [isMobile, isMobileMenuOpen]);

  const handleCloseWalletModal = () => {
    setIsWalletModalOpen(false);
  };
  return (
    <>
      <header className={cx(styles.header)}>
        <div className={cx(styles.container)}>
          <Link to="/">
            <img src={logo} alt="" />
          </Link>
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
          {isMobile ? (
            <div className={cx(styles['burger-menu-button'])}>
              <Button label="" variant="icon" onClick={() => setIsMobileMenuOpen(true)} icon={menuIcon} />
            </div>
          ) : (
            <WalletInfo account={account} />
          )}
        </div>
      </header>
      {isMobileMenuOpen && (
        <>
          <div className={cx(styles['blur-background'])} />
          <BurgerMenu burgerMenuHandler={burgerMenuHandler} />
        </>
      )}

      <WalletModal open={isWalletModalOpen} setOpen={setIsMobileMenuOpen} onClose={handleCloseWalletModal} />
    </>
  );
}

export { Header };
