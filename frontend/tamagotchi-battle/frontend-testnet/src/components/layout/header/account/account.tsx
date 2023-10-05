import { useState } from 'react';
import { useAccount } from '@gear-js/react-hooks';
import { Button, buttonStyles } from '@gear-js/ui';
import { WalletBalance } from 'features/wallet/components/wallet-balance';
import { WalletModal } from 'features/wallet/components/wallet-modal';
import { WalletAccount } from 'features/wallet/components/wallet-account';
import { useBattle } from 'features/battle/context';
import { useBattleMessage } from 'features/battle/hooks';
import { Link, useLocation, useNavigate } from 'react-router-dom';
import { cn } from 'app/utils';

export const AccountComponent = () => {
  const { account } = useAccount();
  const { battle, isAdmin, isPending, setIsPending, setIsAdmin } = useBattle();
  const [isModalOpen, setIsModalOpen] = useState(false);
  const { pathname } = useLocation();
  const navigate = useNavigate();

  const openModal = () => setIsModalOpen(true);
  const closeModal = () => {
    setIsModalOpen(false);
    setIsAdmin(false);
    if (battle?.state === 'Registration') navigate('/');
  };
  const handleMessage = useBattleMessage();

  const onSuccess = () => setIsPending(false);
  const onError = () => setIsPending(false);
  const handler = () => {
    setIsPending(true);
    handleMessage({ StartRegistration: null }, { onSuccess, onError });
  };

  return (
    <div className="flex items-center gap-4">
      {battle?.state === 'GameIsOver' && isAdmin && (
        <Button text="Start New Game" color="primary" onClick={handler} disabled={isPending} />
      )}
      {battle?.state === 'Registration' && isAdmin && pathname !== '/battle' && (
        <Link to="/battle" className={cn('btn transition-colors', buttonStyles.primary)}>
          Battle Page
        </Link>
      )}
      {account ? (
        <div className="flex gap-4">
          <WalletBalance
            balance={account.balance}
            address={account.address}
            name={account.meta.name}
            onClick={openModal}
          />
          <div className="max-w-[260px]">
            <WalletAccount address={account.address} name={account.meta.name} onClick={openModal} simple />
          </div>
        </div>
      ) : (
        <Button text="Connect account" onClick={openModal} color="lightGreen" />
      )}
      {isModalOpen && <WalletModal close={closeModal} />}
    </div>
  );
};
