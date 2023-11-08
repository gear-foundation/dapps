import { useState } from 'react';
import Identicon from '@polkadot/react-identicon';
import { useAtom } from 'jotai';
import { cx, shortenString } from '@/utils';
import { ADDRESS } from '@/consts';
import { CONTRACT_ADDRESS_ATOM } from '@/atoms';
import { WalletInfoProps } from './WalletInfo.interfaces';
import { Button } from '@/ui';
import { WalletModal } from '../WalletModal';
import styles from './WalletInfo.module.scss';

function WalletInfo({ account }: WalletInfoProps) {
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
      {account ? (
        <div className={cx(styles['wallet-info'])}>
          <button className={cx(styles.description)} onClick={handleOpenWalletModal}>
            {address && (
              <Identicon
                value={ADDRESS.FACTORY}
                size={16}
                theme="polkadot"
                className={cx(styles['description-icon'])}
              />
            )}
            <div className={cx(styles['description-name'])}>{shortenString(account?.decodedAddress, 3)}</div>
          </button>
        </div>
      ) : (
        <Button label="connect" variant="outline" onClick={handleOpenWalletModal} />
      )}
      {isWalletModalOpen && <WalletModal onClose={handleCloseWalletModal} />}
    </>
  );
}

export { WalletInfo };
