import { useState } from 'react';
import Identicon from '@polkadot/react-identicon';
import { useAtomValue } from 'jotai';
import { cx } from 'utils';
import { Button } from '@gear-js/ui';
import { CURRENT_CONTRACT_ADDRESS_ATOM } from 'atoms';
import coin from 'assets/images/icons/vara-coin-silver.png';
import { WalletInfoProps } from './WalletInfo.interfaces';

import { WalletModal } from '../wallet-modal';
import styles from './WalletInfo.module.scss';

function WalletInfo({ account }: WalletInfoProps) {
  const address = useAtomValue(CURRENT_CONTRACT_ADDRESS_ATOM);
  const [isWalletModalOpen, setIsWalletModalOpen] = useState<boolean>(false);

  const handleCloseWalletModal = () => {
    setIsWalletModalOpen(false);
  };

  const handleOpenWalletModal = () => {
    setIsWalletModalOpen(true);
  };

  return (
    <>
      {account ? (
        <div className={cx(styles['wallet-info'])}>
          <div className={cx(styles.balance)}>
            <img src={coin} alt="wara coin" className={cx(styles['balance-coin-image'])} />
            <div className={cx(styles['balance-value'])}>{account.balance.value}</div>
            <div className={cx(styles['balance-currency-name'])}>{account.balance.unit}</div>
          </div>
          <button className={cx(styles.description)} onClick={handleOpenWalletModal} type="button">
            {address && (
              <Identicon value={address} size={21} theme="polkadot" className={cx(styles['description-icon'])} />
            )}
            <div className={cx(styles['description-name'])}>{account?.meta.name}</div>
          </button>
        </div>
      ) : (
        <Button text="connect" onClick={handleOpenWalletModal} className={cx(styles.btn)} />
      )}
      {isWalletModalOpen && <WalletModal onClose={handleCloseWalletModal} />}
    </>
  );
}

export { WalletInfo };
