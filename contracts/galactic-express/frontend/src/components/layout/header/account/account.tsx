import { useState } from 'react';
import { useAccount } from '@gear-js/react-hooks';
import { Button } from '@gear-js/ui';
import { GasWallet } from 'components/common/gas-wallet';
import { SelectAccountPopup } from 'components/popups/select-account-popup';
import { AccountButton } from 'components/common/account-button';
import { useApp } from 'app/context';
import { useLaunchMessage } from 'app/hooks/use-battle';
import { useNavigate } from 'react-router-dom';

export const AccountComponent = () => {
  const { account, accounts } = useAccount();
  const { setIsPending } = useApp();
  const [isModalOpen, setIsModalOpen] = useState(false);
  const navigate = useNavigate();

  const openModal = () => setIsModalOpen(true);
  const closeModal = () => setIsModalOpen(false);
  const handleMessage = useLaunchMessage();

  const onSuccess = () => {
    navigate('/');
    setIsPending(false);
  };
  const onError = () => setIsPending(false);
  const handler = () => {
    setIsPending(true);
    handleMessage({ StartNewGame: null }, { onSuccess, onError });
  };

  return (
    <div className="flex items-center gap-4">
      {account ? (
        <div className="flex gap-4">
          <GasWallet balance={account.balance} address={account.address} name={account.meta.name} onClick={openModal} />
          <div className="max-w-[260px]">
            <AccountButton address={account.address} name={account.meta.name} onClick={openModal} />
          </div>
        </div>
      ) : (
        <Button text="Connect account" onClick={openModal} color="lightGreen" />
      )}
      {isModalOpen && <SelectAccountPopup accounts={accounts} close={closeModal} />}
    </div>
  );
};
