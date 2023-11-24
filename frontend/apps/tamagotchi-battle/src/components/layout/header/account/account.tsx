import { useState } from 'react';
import { Button, buttonStyles } from '@gear-js/ui';
import { Wallet } from 'features/wallet';
import { useBattle } from 'features/battle/context';
import { useBattleMessage } from 'features/battle/hooks';
import { Link, useLocation, useNavigate } from 'react-router-dom';
import { cn } from 'app/utils';

export const AccountComponent = () => {
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
    const payload = { StartRegistration: null };

    setIsPending(true);
    handleMessage({ payload, onSuccess, onError });
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
      <Wallet isModalOpen={isModalOpen} openModal={openModal} closeModal={closeModal} />
    </div>
  );
};
