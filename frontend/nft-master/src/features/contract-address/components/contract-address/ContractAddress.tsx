import { useState } from 'react';
import { ReactComponent as EditSVG } from 'assets/images/icons/edit.svg';
import { useContractAddress } from 'features/contract-address/hooks';
import { ContractAddressModal } from '../contract-address-modal';
import styles from './ContractAddress.module.scss';

function ContractAddress() {
  const { contractAddress } = useContractAddress();
  const [isModalOpen, setIsModalOpen] = useState(false);

  const openModal = () => setIsModalOpen(true);
  const closeModal = () => setIsModalOpen(false);

  return contractAddress ? (
    <>
      <button type="button" className={styles.button} onClick={openModal}>
        <span>{contractAddress}</span>
        <EditSVG />
      </button>

      {isModalOpen && <ContractAddressModal onClose={closeModal} />}
    </>
  ) : null;
}

export { ContractAddress };
