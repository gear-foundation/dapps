import { buttonStyles } from '@gear-js/ui';
import { Wallet } from '@dapps-frontend/ui';
import { useBattle } from 'features/battle/context';
import { Link, useLocation } from 'react-router-dom';
import { cn } from 'app/utils';
import { NewGameButton } from 'features/battle/components/new-game-button';

export const AccountComponent = () => {
  const { battle, isAdmin } = useBattle();
  const { pathname } = useLocation();

  return (
    <div className="flex items-center gap-4">
      {battle?.state === 'GameIsOver' && isAdmin && <NewGameButton />}
      {battle?.state === 'Registration' && isAdmin && pathname !== '/battle' && (
        <Link to="/battle" className={cn('btn transition-colors', buttonStyles.primary)}>
          Battle Page
        </Link>
      )}

      <Wallet variant="gear" />
    </div>
  );
};
