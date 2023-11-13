import { useState } from 'react';
import Identicon from '@polkadot/react-identicon';
import { useAtom } from 'jotai';
import { cx } from '@/utils';
import { ADDRESS } from '@/consts';
import { CONTRACT_ADDRESS_ATOM } from '@/atoms';
import varaCoin from '@/assets/icons/vara-coin.svg';
import tVaraCoin from '@/assets/icons/tvara-coin.svg';
import { WalletInfoProps } from './WalletInfo.interfaces';
import { Button } from '@/components/ui';
import { WalletModal } from '../WalletModal';
import styles from './WalletInfo.module.scss';
import { useAccountAvailableBalance } from '../../hooks';

function WalletInfo({ account, withoutBalance, buttonClassName }: WalletInfoProps) {
  const address = useAtom(CONTRACT_ADDRESS_ATOM);
  const { availableBalance: balance, isAvailableBalanceReady } = useAccountAvailableBalance();
  const [isWalletModalOpen, setIsWalletModalOpen] = useState<boolean>(false);

  const handleCloseWalletModal = () => {
    setIsWalletModalOpen(false);
  };

  const handleOpenWalletModal = () => {
    setIsWalletModalOpen(true);
  };

  const balanceValue = (balance?.value || '0').split('.');
  const balanceAmount = balanceValue[0].replaceAll(/,|\s/g, '&thinsp;');
  const balanceDecimals = balanceValue[1];

  return (
    <>
      {account && isAvailableBalanceReady ? (
        <div className={cx(styles['wallet-info'])}>
          {!withoutBalance && (
            <div className={cx(styles.balance)}>
              <img
                src={balance?.unit?.toLowerCase() === 'vara' ? varaCoin : tVaraCoin}
                alt="vara coin"
                className={cx(styles['balance-coin-image'])}
              />
              <div className={cx(styles['balance-value'])}>{balanceAmount}</div>
              {balanceDecimals && <div className={cx(styles['balance-decimals'])}>{`.${balanceDecimals}`}</div>}
              <div className={cx(styles['balance-currency-name'])}>{balance?.unit}</div>
            </div>
          )}
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
