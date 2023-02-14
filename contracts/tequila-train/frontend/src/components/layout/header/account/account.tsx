import { useState } from 'react';
import { useAccount } from '@gear-js/react-hooks';
import { Button, buttonStyles } from '@gear-js/ui';
import { GasWallet } from 'components/common/gas-wallet';
import { SelectAccountPopup } from 'components/popups/select-account-popup';
import clsx from 'clsx';
import Identicon from '@polkadot/react-identicon';

export const AccountComponent = () => {
  const { account, accounts } = useAccount();
  const [isModalOpen, setIsModalOpen] = useState(false);

  const openModal = () => setIsModalOpen(true);
  const closeModal = () => setIsModalOpen(false);

  return (
    <div className="flex items-center gap-4">
      {account ? (
        <div className="flex gap-4">
          <GasWallet balance={account.balance} address={account.address} name={account.meta.name} onClick={openModal} />
          <div className="max-w-[260px]">
            <button
              className={clsx('btn btn--primary gap-2 w-full !justify-start', buttonStyles.button)}
              onClick={openModal}>
              <Identicon value={account.address} className={buttonStyles.icon} theme="polkadot" size={20} />
              <span className="truncate">{account.meta.name}</span>
            </button>
          </div>
        </div>
      ) : (
        <Button text="Connect account" onClick={openModal} color="lightGreen" />
      )}
      {isModalOpen && <SelectAccountPopup accounts={accounts} close={closeModal} />}
    </div>
  );
};
