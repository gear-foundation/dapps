import { useState } from 'react';
import { Container } from 'components';
import { WalletModal } from '../wallet';
import styles from './Welcome.module.scss';

function Welcome() {
  const [isWalletModalOpen, setIsWalletModalOpen] = useState(false);

  const openWalletModal = () => setIsWalletModalOpen(true);
  const closeWalletModal = () => setIsWalletModalOpen(false);

  return (
    <>
      <Container className={styles.container}>
        <div className={styles.welcome}>
          <h2 className={styles.heading}>
            Vara <span className={styles.nftText}>NFT</span>
          </h2>

          <p className={styles.text}>
            A simple application that shows user&apos;s NFTs in different gear networks and contracts
          </p>

          <button type="button" onClick={openWalletModal} className={styles.button}>
            Connect Account
          </button>
        </div>
      </Container>

      {isWalletModalOpen && <WalletModal onClose={closeWalletModal} />}
    </>
  );
}

export { Welcome };
