import clsx from 'clsx';
import { AnimatePresence } from 'framer-motion';
import { useState } from 'react';

import { Container } from '@/components/ui/container';
import { WalletChange, WalletConnect } from '@/features/wallet/components';

import ModalBackground from './ModalBackground';
import { AccountInfo } from './account-info';
import styles from './header.module.scss';
import { Logo } from './logo';

export function Header() {
  const [isOpenChange, setIsOpenChange] = useState(false);
  const openAndCloseChange = () => setIsOpenChange(!isOpenChange);
  const closeChange = () => setIsOpenChange(false);

  const [isOpenConnectWallet, setIsOpenConnectWallet] = useState(false);
  const openConnectWallet = () => setIsOpenConnectWallet(true);
  const closConnectWallet = () => setIsOpenConnectWallet(false);

  return (
    <>
      <header className={clsx(styles.header)}>
        <Container className={styles.header__container}>
          <Logo className={styles.header__logo} />
          <AccountInfo
            openWallet={openAndCloseChange}
            openConnectWallet={openConnectWallet}
            isOpen={isOpenChange}
            className={clsx(styles.accountInfo)}
          />
        </Container>
        {isOpenChange && (
          <Container>
            <WalletChange onClose={openAndCloseChange} openConnectWallet={openConnectWallet} />
          </Container>
        )}
      </header>

      <ModalBackground isOpen={isOpenChange} onClick={closeChange} />

      <AnimatePresence>{isOpenConnectWallet && <WalletConnect onClose={closConnectWallet} />}</AnimatePresence>
    </>
  );
}
