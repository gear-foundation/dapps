import Identicon from '@polkadot/react-identicon';
import cx from 'clsx';
import styles from './wallet.module.css';
import { useState } from 'react';
import { useAccount } from '@gear-js/react-hooks';
import { WalletModal } from '../wallet-modal';

function Wallet() {
  const { account, isAccountReady } = useAccount();

  const [isModalOpen, setIsModalOpen] = useState(false);
  const openModal = () => setIsModalOpen(true);
  const closeModal = () => setIsModalOpen(false);

  const buttonClassName = cx(styles.button, account && styles.active);

  return isAccountReady ? (
    <>
      <button type="button" className={buttonClassName} onClick={openModal}>
        {account && <Identicon value={account.address} size={16} theme="polkadot" />}
        <span>{account ? account.meta.name : 'Connect'}</span>
      </button>

      {isModalOpen && <WalletModal close={closeModal} />}
    </>
  ) : null;
}

export { Wallet };
