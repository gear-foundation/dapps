import { useAtom } from 'jotai';
import { useState } from 'react';
import { ReactComponent as EditSVG } from 'assets/images/icons/edit.svg';
import { CONTRACT_ADDRESS_ATOM } from '../../consts';
import { ContractAddressModal } from '../contract-address-modal';
import styles from './ContractAddress.module.scss';

function ContractAddress() {
  const [address] = useAtom(CONTRACT_ADDRESS_ATOM);
  const [isModalOpen, setIsModalOpen] = useState(false);

  const openModal = () => setIsModalOpen(true);
  const closeModal = () => setIsModalOpen(false);

  return address ? (
    <>
      <button type="button" className={styles.button} onClick={openModal}>
        <span>{address}</span>
        <EditSVG />
      </button>

      {isModalOpen && <ContractAddressModal onClose={closeModal} />}
    </>
  ) : null;
}

export { ContractAddress };
