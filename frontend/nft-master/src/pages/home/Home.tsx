import { Button } from '@gear-js/ui';
import { useState } from 'react';
import { WalletModal } from 'features/wallet';
import { useAccount } from '@gear-js/react-hooks';
import styles from './Home.module.scss';

function Home() {
  const { account } = useAccount();

  const [isWalletModalOpen, setIsWalletModalOpen] = useState(false);

  const openWalletModal = () => setIsWalletModalOpen(true);
  const closeWalletModal = () => setIsWalletModalOpen(false);

  return account ? null : (
    <div className={styles.welcome}>
      <h2 className={styles.heading}>
        Vara <span className={styles.nftText}>NFT</span>
      </h2>

      <p className={styles.text}>
        A simple application that shows user&apos;s NFTs in different gear networks and contracts
      </p>

      <Button text="Connect Account" className={styles.button} onClick={openWalletModal} />

      {isWalletModalOpen && <WalletModal onClose={closeWalletModal} />}
    </div>
  );
}

export { Home };
