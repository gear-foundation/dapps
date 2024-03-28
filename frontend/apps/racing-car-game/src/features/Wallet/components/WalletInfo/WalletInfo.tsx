import { useState } from 'react';
import Identicon from '@polkadot/react-identicon';
import { VaraBalanceNew as VaraBalance } from '@dapps-frontend/ui';
import { useAtom } from 'jotai';
import { cx } from '@/utils';
import { ADDRESS } from '@/consts';
import { CONTRACT_ADDRESS_ATOM } from '@/atoms';
import { WalletInfoProps } from './WalletInfo.interfaces';
import { Button } from '@/ui';
import { WalletModal } from '../WalletModal';
import styles from './WalletInfo.module.scss';

function WalletInfo({ account, withoutBalance, buttonClassName }: WalletInfoProps) {
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
          {!withoutBalance && <VaraBalance />}
          <button className={cx(styles.description)} onClick={handleOpenWalletModal}>
            {address && (
              <Identicon
                value={ADDRESS.CONTRACT}
                size={21}
                theme="polkadot"
                className={cx(styles['description-icon'])}
              />
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
