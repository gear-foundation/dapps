import { Logo } from './logo';
import styles from './header.module.scss';
import { Container } from '@/components/ui/container';
import clsx from 'clsx';
import { useAccount } from '@gear-js/react-hooks';
import { MobileMenu } from '@/components/layout/header/mobile-menu';
import { Wallet } from '@/features/wallet';
import { Suspense } from 'react';
import { Loader } from '@/components/loaders';
import { VaraBalance } from '@/components/ui/balance';
import { useAccountAvailableBalance } from '@/features/account-available-balance/hooks';

export function Header() {
  const { account } = useAccount();
  const { availableBalance: balance } = useAccountAvailableBalance();

  return (
    <header className={styles.header}>
      <Container className={styles.header__container}>
        <Logo className={clsx(styles.header__logo, !account && styles['header__logo--center'])} label="Tic-Tac-Toe" />
        <div className={styles.menu_wrapper}>
          {!!account && (
            <>
              <Suspense fallback={<Loader />}>
                <MobileMenu />
              </Suspense>
              <VaraBalance value={balance?.value || '0'} unit={balance?.unit} className={styles.mobile_balance} />
            </>
          )}
        </div>
        <div className={styles.header__wallet}>
          <Wallet className={styles.wallet} />
        </div>
      </Container>
    </header>
  );
}
