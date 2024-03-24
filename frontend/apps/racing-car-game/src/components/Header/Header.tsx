import { useEffect, useState } from 'react';
import { useLocation } from 'react-router-dom';
import { useAccount, useApi, useAccountDeriveBalancesAll, useBalanceFormat } from '@gear-js/react-hooks';
import { VaraBalanceNew as VaraBalance } from '@dapps-frontend/ui';
import { cx } from '@/utils';
import { Link } from '@/ui';
import styles from './Header.module.scss';
import logo from '@/assets/icons/logo-vara-black.svg';
import { HeaderProps } from './Header.interfaces';
import { useMediaQuery } from '@/hooks';
import { WalletInfo } from '@/features/Wallet/components/WalletInfo';
import { MobileMenu } from '../MobileMenu';

function Header({ menu }: HeaderProps) {
  const location = useLocation();
  const { isApiReady } = useApi();
  const { account } = useAccount();
  const isMobile = useMediaQuery(768);
  const balances = useAccountDeriveBalancesAll();
  const { getFormattedBalance } = useBalanceFormat();
  const [isMobileMenuOpen, setIsMobileMenuOpen] = useState<boolean>(false);
  const balance =
    isApiReady && balances?.freeBalance ? getFormattedBalance(balances.freeBalance.toString()) : undefined;

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
          <Link to="/" className={cx(styles['logo-link'], !account ? styles['logo-link-centered'] : '')}>
            <img src={logo} alt="" />
            <span className={cx(styles['post-logo'])}>Racing Car</span>
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
                                location.pathname === `/${url}` ? styles['menu-item--active'] : '',
                              )}>
                              {item}
                            </p>
                          </Link>
                        );
                      })}
                  </nav>
                </>
              )}
            </>
          )}
          {!isMobile && <WalletInfo account={account} buttonClassName={cx(styles['wallet-info-connect-btn'])} />}
          {account && isMobile && (
            <div className={cx(styles['menu-wrapper'])}>
              <VaraBalance />
              {!!account && <MobileMenu />}
            </div>
          )}
        </div>
      </header>
    </>
  );
}

export { Header };
