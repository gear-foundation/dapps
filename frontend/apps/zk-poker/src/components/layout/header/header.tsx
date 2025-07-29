import clsx from 'clsx';
import { AnimatePresence } from 'framer-motion';
import { useState } from 'react';

import { WalletChange, WalletConnect } from '@/features/wallet/components';

import ModalBackground from './ModalBackground';
import { AccountInfo } from './account-info';
import styles from './header.module.scss';
import { Logo } from './logo';

type HeaderProps = BaseComponentProps;

export function Header({ children }: HeaderProps) {
  const [isOpenChange, setIsOpenChange] = useState(false);
  const openAndCloseChange = () => setIsOpenChange(!isOpenChange);
  const closeChange = () => setIsOpenChange(false);

  const [isOpenConnectWallet, setIsOpenConnectWallet] = useState(false);
  const openConnectWallet = () => setIsOpenConnectWallet(true);
  const closConnectWallet = () => setIsOpenConnectWallet(false);

  return (
    <>
      <header className={clsx(styles.header, isOpenChange && styles.open)}>
        <div className={styles.header__container}>
          {isOpenChange ? <Logo className={styles.header__logo} /> : children}
          <AccountInfo openWallet={openAndCloseChange} isOpen={isOpenChange} className={styles.accountInfo} />
        </div>

        {isOpenChange && <WalletChange onClose={openAndCloseChange} openConnectWallet={openConnectWallet} />}
      </header>

      <ModalBackground isOpen={isOpenChange} onClick={closeChange} />

      <AnimatePresence>{isOpenConnectWallet && <WalletConnect onClose={closConnectWallet} />}</AnimatePresence>
    </>
  );
}
