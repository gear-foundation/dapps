import { useAccount } from '@gear-js/react-hooks';
import clsx from 'clsx';
import { useState } from 'react';

import { Button, Sprite } from '@/components';
import { Search } from '@/features/nfts';
import { Wallet } from '@/features/wallet';
import { useResizeEffect } from '@/hooks';

import { useIsAppReady } from '../../../app/hooks/use-is-app-ready';
import { AccountBalance } from '../../ui/balance/Balance';
import { Container } from '../container';

import styles from './Header.module.scss';
import { Logo } from './logo';

export function Header() {
  const [isMenuOpen, setIsMenuOpen] = useState(false);

  const { isAppReady } = useIsAppReady();
  const { account } = useAccount();

  const toggleMenu = () => setIsMenuOpen((prevValue) => !prevValue);
  const closeMenu = () => setIsMenuOpen(false);

  useResizeEffect(closeMenu);

  return (
    <header>
      <Container className={styles.container}>
        <Logo />

        <div className={styles.mobileMenuWrapper}>
          <Button variant="white" className={styles.button} onClick={toggleMenu}>
            <Sprite name={isMenuOpen ? 'close' : 'burger-menu'} width={25} height={24} />
          </Button>

          {isMenuOpen && (
            <ul className={styles.list}>
              {account && (
                <li className={styles.item}>
                  <AccountBalance className={styles.balance} />
                </li>
              )}
              <li className={clsx(styles.item, styles['item--wallet'])}>
                <Wallet />
              </li>
            </ul>
          )}
        </div>

        <div className={styles.configuration}>
          {isAppReady && <Search />}

          <div className={styles.desktopMenu}>
            {isAppReady && <span className={styles.separator} />}

            <div className={styles.desktopWallet}>
              {isAppReady && !!account && <AccountBalance className={styles.balance} />}

              <Wallet />
            </div>
          </div>
        </div>
      </Container>
    </header>
  );
}
