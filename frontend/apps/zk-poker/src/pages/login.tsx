import { AnimatePresence } from 'framer-motion';
import { useState } from 'react';

import { LoginImage, LoginLogo, VaraLogoIcon } from '@/assets/images';
import { Button, Footer } from '@/components';
import { WalletConnect } from '@/features/wallet';

import styles from './login.module.scss';

export default function Login() {
  const [isOpen, setIsOpen] = useState(false);
  const openWallet = () => setIsOpen(true);
  const closeWallet = () => setIsOpen(false);

  return (
    <div className={styles.container}>
      <div className={styles.glow} />
      <div className={styles.content}>
        <div className={styles.logo}>
          <VaraLogoIcon />
        </div>

        <div className={styles.image}>
          <LoginImage />
          <LoginLogo className={styles.spade} />
        </div>

        <h1 className={styles.title}>Welcome to ZK Poker</h1>
        <p className={styles.description}>To get started, connect your wallet.</p>

        <Button className={styles.button} onClick={openWallet}>
          Connect wallet
        </Button>
      </div>

      <Footer />

      <AnimatePresence>{isOpen && <WalletConnect onClose={closeWallet} />}</AnimatePresence>
    </div>
  );
}
