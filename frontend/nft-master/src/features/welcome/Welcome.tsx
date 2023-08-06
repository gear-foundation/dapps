import { useState } from 'react';
import { Button, Container, Heading, Text, TextGradient, textVariants } from 'components';
import clsx from 'clsx';
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
          <Heading size="xl" className={styles.heading}>
            Vara <TextGradient>NFT</TextGradient>
          </Heading>

          <div className={textVariants({ size: 'lg', className: styles.text })}>
            <p>A simple application that shows user&apos;s NFTs in different gear networks and contracts</p>
          </div>

          <Button onClick={openWalletModal}>Connect Account</Button>
        </div>
      </Container>

      {isWalletModalOpen && <WalletModal onClose={closeWalletModal} />}
    </>
  );
}

export { Welcome };
