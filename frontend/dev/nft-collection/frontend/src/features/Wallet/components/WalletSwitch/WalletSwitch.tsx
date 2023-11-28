import { useState } from 'react';
import Identicon from '@polkadot/react-identicon';
import { useAtom } from 'jotai';
import { cx, shortenString } from '@/utils';
import { ADDRESS } from '@/consts';
import { CONTRACT_ADDRESS_ATOM } from '@/atoms';
import coin from '@/assets/icons/vara-coin-silver.png';
import { WalletSwitchProps } from './WalletSwitch.interfaces';
import { Button } from '@/ui';
import { WalletModal } from '../WalletModal';
import styles from './WalletSwitch.module.scss';

function WalletSwitch({ children }: WalletSwitchProps) {
  const address = useAtom(CONTRACT_ADDRESS_ATOM);
  const [isWalletModalOpen, setIsWalletModalOpen] = useState<boolean>(false);

  const handleCloseWalletModal = () => {
    setIsWalletModalOpen(false);
  };

  const handleOpenWalletModal = () => {
    setIsWalletModalOpen(true);
  };

  return (
    <>
      <button type="button" className={cx(styles['invisible-button'])} onClick={handleOpenWalletModal}>
        {children}
      </button>
      {isWalletModalOpen && <WalletModal onClose={handleCloseWalletModal} />}
    </>
  );
}

export { WalletSwitch };
