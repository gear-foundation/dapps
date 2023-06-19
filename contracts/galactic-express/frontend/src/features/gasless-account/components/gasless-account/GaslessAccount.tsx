import { buttonStyles } from '@gear-js/ui';
import Identicon from '@polkadot/react-identicon';
import clsx from 'clsx';
import { useState } from 'react';
import { WalletSVG } from '../../../wallet/assets';
import { useGaslessAccount } from '../../Context';
import { GaslessAccountModal } from '../gasless-account-modal';
import styles from './GaslessAccount.module.scss';

function GaslessAccount() {
  const { isLoggedIn, account } = useGaslessAccount();

  const [isModalOpen, setIsModalOpen] = useState(false);

  const openModal = () => setIsModalOpen(true);
  const closeModal = () => setIsModalOpen(false);

  const className = clsx(buttonStyles.button, buttonStyles.primary, buttonStyles.medium);
  const activeClassName = clsx(styles.button, buttonStyles.button, buttonStyles.light, buttonStyles.medium);

  return (
    <>
      {isLoggedIn ? (
        <button type="button" className={activeClassName} onClick={openModal}>
          <Identicon value={account.publicKey} size={16} theme="polkadot" className={buttonStyles.icon} />
          <span>{account.publicKey}</span>
        </button>
      ) : (
        <button type="button" className={className} onClick={openModal}>
          <WalletSVG className={buttonStyles.icon} />
          <span>Login</span>
        </button>
      )}

      {isModalOpen && <GaslessAccountModal onClose={closeModal} />}
    </>
  );
}

export { GaslessAccount };
