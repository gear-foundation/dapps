import { useAccount } from '@gear-js/react-hooks';
import { Wallet } from '@gear-js/wallet-connect';
import clsx from 'clsx';
import { Navigate } from 'react-router-dom';

import { ROUTES } from '@/app/consts';
import { LoginImage, LoginLogo, VaraLogoIcon } from '@/assets/images';
import { Footer } from '@/components';

import styles from './login.module.scss';

export default function Login() {
  const { account } = useAccount();

  if (account) {
    return <Navigate to={ROUTES.HOME} />;
  }

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

        <div className={clsx(styles.wallet)}>
          <Wallet theme="vara" displayBalance={false} />
        </div>
      </div>

      <Footer />
    </div>
  );
}
