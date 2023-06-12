import { useAccount } from '@gear-js/react-hooks';
import Identicon from '@polkadot/react-identicon';
import clsx from 'clsx';
import { useState } from 'react';
import { buttonStyles } from '@gear-js/ui';
import { WalletSVG } from '../../assets';
import { WalletModal } from '../wallet-modal';

function Wallet() {
  const { account, isAccountReady } = useAccount();

  const [isModalOpen, setIsModalOpen] = useState(false);

  const openModal = () => setIsModalOpen(true);
  const closeModal = () => setIsModalOpen(false);

  const className = clsx(buttonStyles.button, buttonStyles.primary, buttonStyles.medium);
  const activeClassName = clsx(buttonStyles.button, buttonStyles.light, buttonStyles.medium);

  return isAccountReady ? (
    <>
      {account ? (
        <button type="button" className={activeClassName} onClick={openModal}>
          <Identicon value={account.address} size={16} theme="polkadot" className={buttonStyles.icon} />
          <span>{account.meta.name}</span>
        </button>
      ) : (
        <button type="button" className={className} onClick={openModal}>
          <WalletSVG className={buttonStyles.icon} />
          <span>Connect</span>
        </button>
      )}

      {isModalOpen && <WalletModal onClose={closeModal} />}
    </>
  ) : null;
}

export { Wallet };
