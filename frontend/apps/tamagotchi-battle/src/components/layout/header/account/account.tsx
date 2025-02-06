import { buttonStyles } from '@gear-js/ui';
import { Link, useLocation } from 'react-router-dom';

import { Wallet } from '@dapps-frontend/ui';

import { cn } from '@/app/utils';
import { NewGameButton } from '@/features/battle/components/new-game-button';
import { useBattle } from '@/features/battle/context';

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

      <Wallet theme="gear" />
    </div>
  );
};
