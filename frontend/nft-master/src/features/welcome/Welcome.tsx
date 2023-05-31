import { Button } from '@gear-js/ui';
import { useAccount } from '@gear-js/react-hooks';
import { useState } from 'react';
import { Container } from 'components';
import { WalletModal } from '../wallet';
import { useContractAddress, ContractAddressModal } from '../contract-address';
import styles from './Welcome.module.scss';

function Welcome() {
  const { account } = useAccount();
  const contractAddress = useContractAddress();

  const [isWalletModalOpen, setIsWalletModalOpen] = useState(false);
  const [isContractModalOpen, setIsContractModalOpen] = useState(false);

  const openWalletModal = () => setIsWalletModalOpen(true);
  const closeWalletModal = () => setIsWalletModalOpen(false);
  const openContractModal = () => setIsContractModalOpen(true);
  const closeContractModal = () => setIsContractModalOpen(false);

  const buttonText = account ? 'Connect Contract' : 'Connect Account';
  const handleButtonClick = account ? openContractModal : openWalletModal;

  return (
    <>
      <Container>
        <div className={styles.welcome}>
          <h2 className={styles.heading}>
            Vara <span className={styles.nftText}>NFT</span>
          </h2>

          <p className={styles.text}>
            A simple application that shows user&apos;s NFTs in different gear networks and contracts
          </p>

          {!contractAddress && <Button text={buttonText} onClick={handleButtonClick} className={styles.button} />}
        </div>
      </Container>

      {isWalletModalOpen && <WalletModal onClose={closeWalletModal} onSelect={openContractModal} />}
      {isContractModalOpen && <ContractAddressModal onClose={closeContractModal} />}
    </>
  );
}

export { Welcome };
