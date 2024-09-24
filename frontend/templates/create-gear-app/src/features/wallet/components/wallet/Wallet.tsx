import { useAccount } from '@gear-js/react-hooks';
import { buttonStyles } from '@gear-js/ui';
import { Identicon } from '@polkadot/react-identicon';
import clsx from 'clsx';
import { useState } from 'react';

import { WalletSVG } from '../../assets';
import { Balance } from '../balance';
import { WalletModal } from '../wallet-modal';

import styles from './Wallet.module.scss';

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
        <div className={styles.wallet}>
          <Balance />

          <button type="button" className={activeClassName} onClick={openModal}>
            <Identicon value={account.address} size={16} theme="polkadot" className={buttonStyles.icon} />
            <span>{account.meta.name}</span>
          </button>
        </div>
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
