import Identicon from '@polkadot/react-identicon';
import { useState } from 'react';

import { useDnsProgramIds } from '@dapps-frontend/hooks';

import { Button } from '@/ui';
import { cx } from '@/utils';

import { WalletModal } from '../WalletModal';

import { WalletInfoProps } from './WalletInfo.interfaces';
import styles from './WalletInfo.module.scss';

function WalletInfo({ account, buttonClassName }: WalletInfoProps) {
  const { programId } = useDnsProgramIds();
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
            {programId && (
              <Identicon value={programId} size={21} theme="polkadot" className={cx(styles['description-icon'])} />
            )}
            <div className={cx(styles['description-name'])}>{account?.meta.name}</div>
          </button>
        </div>
      ) : (
        <Button
          label="Connect Wallet"
          variant="primary"
          size="large"
          onClick={handleOpenWalletModal}
          className={cx(styles['connect-btn'], buttonClassName || '')}
        />
      )}
      <WalletModal open={isWalletModalOpen} setOpen={setIsWalletModalOpen} onClose={handleCloseWalletModal} />
    </>
  );
}

export { WalletInfo };
