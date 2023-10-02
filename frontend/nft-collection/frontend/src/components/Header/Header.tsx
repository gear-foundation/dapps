import { useEffect, useState } from 'react';
import { useLocation } from 'react-router-dom';
import { Button, Link } from '@ui';
import { useAccount } from '@gear-js/react-hooks';
import { cx } from '@/utils';
import styles from './Header.module.scss';
import logo from '@/assets/icons/logo-vara-nft.svg';
import { HeaderProps } from './Header.interfaces';
import { useMediaQuery } from '@/hooks';
import coin from '@/assets/icons/vara-coin-silver.png';
import { MobileMenu } from '../MobileMenu';
import { WalletInfo } from '@/features/Wallet/components/WalletInfo';
import { ContractInfo } from '@/features/Auth/components';
import { CREATE_COLLECTION } from '@/routes';
import { Search } from '../Search';
import { useAccountAvailableBalance } from '@/features/Wallet/hooks';
import { SearchModal } from '../SearchModal';

function Header({ menu }: HeaderProps) {
  const location = useLocation();
  const { account } = useAccount();
  const isMobile = useMediaQuery(992);
  const [isMobileMenuOpen, setIsMobileMenuOpen] = useState<boolean>(false);
  const { availableBalance: balance, isAvailableBalanceReady } = useAccountAvailableBalance();

  const burgerMenuHandler = () => {
    setIsMobileMenuOpen(false);
  };

  useEffect(() => {
    if (isMobileMenuOpen && !isMobile) {
      burgerMenuHandler();
    }
  }, [isMobile, isMobileMenuOpen]);

  return (
    <>
      <header className={cx(styles.header)}>
        <div className={cx(styles.container)}>
          <Link to="/" className={cx(styles['logo-link'])}>
            <img src={logo} alt="" />
          </Link>
          {account && (
            <>
              {!isMobile && (
                <>
                  <nav className={cx(styles.menu)}>
                    {menu &&
                      Object.keys(menu).map((item) => {
                        const { url } = menu[item];

                        return (
                          <Link to={url} key={item}>
                            <p
                              className={cx(
                                styles['menu-item'],
                                location.pathname === `${url}` ? styles['menu-item--active'] : '',
                              )}>
                              {item}
                            </p>
                          </Link>
                        );
                      })}
                  </nav>
                  <div className={cx(styles.items)}>
                    <div className={cx(styles['search-wrapper'])}>
                      <Search />
                    </div>
                    {account && (
                      <Link to={CREATE_COLLECTION}>
                        <Button variant="primary" label="Create" className={cx(styles['create-btn'])} />
                      </Link>
                    )}
                    <WalletInfo account={account} withoutBalance />
                    <ContractInfo />
                  </div>
                </>
              )}
            </>
          )}
          {account && isMobile && (
            <div className={cx(styles['menu-wrapper'])}>
              {isAvailableBalanceReady && (
                <div className={cx(styles.balance)}>
                  <img src={coin} alt="vara coin" className={cx(styles['balance-coin-image'])} />
                  <div className={cx(styles['balance-value'])}>{balance?.value || '0'}</div>
                  <div className={cx(styles['balance-currency-name'])}>{account.balance.unit}</div>
                </div>
              )}
              <SearchModal />
              <MobileMenu />
            </div>
          )}
        </div>
      </header>
    </>
  );
}

export { Header };
