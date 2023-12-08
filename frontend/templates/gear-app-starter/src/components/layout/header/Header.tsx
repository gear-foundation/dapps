import { Wallet } from 'features/wallet';
import { Suspense } from 'react';
import { Loader } from 'components/loaders';
import { VaraBalance } from 'components/ui/balance';
import { useAccountAvailableBalance } from 'features/account-available-balance/hooks';
import { MobileMenu } from './mobile-menu';
import { Logo } from './logo';
import styles from './Header.module.scss';

type Props = {
  isAccountVisible: boolean;
};

function Header({ isAccountVisible }: Props) {
  const { availableBalance: balance } = useAccountAvailableBalance();

  return (
    <header className={styles.header}>
      <Logo />
      {isAccountVisible && (
        <div>
          <div className={styles.header__wallet}>
            <Wallet className={styles.wallet} />
          </div>
          <div className={styles.menu_wrapper}>
            <Suspense fallback={<Loader />}>
              <MobileMenu />
            </Suspense>
            <VaraBalance
              value={balance?.value || '0'}
              unit={balance?.unit}
              className={styles.mobile_balance}
            />
          </div>
        </div>
      )}
    </header>
  );
}

export { Header };
