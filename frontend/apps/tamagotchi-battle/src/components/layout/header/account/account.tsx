import { Button, buttonStyles } from '@gear-js/ui';
import { Wallet } from '@dapps-frontend/ui';
import { useBattle } from 'features/battle/context';
import { useBattleMessage } from 'features/battle/hooks';
import { Link, useLocation } from 'react-router-dom';
import { cn } from 'app/utils';

export const AccountComponent = () => {
  const { battle, isAdmin, isPending, setIsPending } = useBattle();
  const { pathname } = useLocation();

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

      <Wallet />
    </div>
  );
};
