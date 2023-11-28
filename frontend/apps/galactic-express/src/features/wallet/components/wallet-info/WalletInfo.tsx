import { useState } from 'react';
import Identicon from '@polkadot/react-identicon';
import { cx } from 'utils';
import { Button } from 'components/layout/button';
import varaCoin from '../../assets/icons/vara-coin.svg';
import tVaraCoin from '../../assets/icons/tvara-coin.svg';
import { WalletInfoProps } from './WalletInfo.interfaces';

import { WalletModal } from '../wallet-modal';
import styles from './WalletInfo.module.scss';
import { useAccountAvailableBalance } from '../../hooks';

function WalletInfo({ account, withoutBalance, buttonClassName }: WalletInfoProps) {
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
              <div className={cx(styles['balance-value'])} dangerouslySetInnerHTML={{ __html: balanceAmount }} />
              {balanceDecimals && <div className={cx(styles['balance-value'])}>{`.${balanceDecimals}`}</div>}
              <div className={cx(styles['balance-currency-name'])}>{balance?.unit}</div>
            </div>
          )}
          <button className={cx(styles.description)} onClick={handleOpenWalletModal}>
            {account.address && (
              <Identicon
                value={account.address}
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
