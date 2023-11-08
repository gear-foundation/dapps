import { useState } from 'react'
import { AnimatePresence } from 'framer-motion'

import { Container } from '@/components/ui/container'
import { WalletChange, WalletConnect } from '@/features/wallet/components'

import { AccountInfo } from './account-info'
import ModalBackground from './ModalBackground'
import { Logo } from './logo'

import styles from './header.module.scss'

export function Header() {
  const [isOpenChange, setIsOpenChange] = useState(false);
  const openAndCloseChange = () => setIsOpenChange(!isOpenChange);
  const closeChange = () => setIsOpenChange(false);

  const [isOpenConnectWallet, setIsOpenConnectWallet] = useState(false)
  const openConnectWallet = () => setIsOpenConnectWallet(true)
  const closConnectWallet = () => setIsOpenConnectWallet(false)

  return (
    <>
      <header className={styles.header}>
        <Container className={styles.header__container}>
          <Logo className={styles.header__logo} />
          <AccountInfo openWallet={openAndCloseChange} isOpen={isOpenChange} />
        </Container>
        {isOpenChange &&
          <Container>
            <WalletChange onClose={openAndCloseChange} openConnectWallet={openConnectWallet} />
          </Container>
        }
      </header>

      <ModalBackground isOpen={isOpenChange} onClick={closeChange} />

      <AnimatePresence>
        {isOpenConnectWallet && <WalletConnect onClose={closConnectWallet} />}
      </AnimatePresence>
    </>
  );
}

