import Identicon from '@polkadot/react-identicon';
import clsx from 'clsx';
import { useState } from 'react';
import { useAccount } from '@gear-js/react-hooks';
import styles from './Wallet.module.scss';
import { WalletModal } from '../wallet-modal';

function Wallet() {
  const { account, isAccountReady } = useAccount();

  const [isWalletModalOpen, setIsWalletModalOpen] = useState(false);

  const openWalletModal = () => setIsWalletModalOpen(true);
  const closeWalletModal = () => setIsWalletModalOpen(false);

  const buttonClassName = clsx(styles.button, account && styles.active);

  return isAccountReady ? (
    <>
      <button type="button" className={buttonClassName} onClick={openWalletModal}>
        {account && <Identicon value={account.address} size={16} theme="polkadot" />}
        <span>{account ? account.meta.name : 'Connect'}</span>
      </button>

      {isWalletModalOpen && <WalletModal onClose={closeWalletModal} />}
    </>
  ) : null;
}

export { Wallet };
