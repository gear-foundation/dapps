import { useAccount } from '@gear-js/react-hooks';
import { Link } from 'react-router-dom';
import { useGame } from '@/app/context/ctx-game';
import { HeaderAdmin } from '@/components/layout/header/header-admin';
import { Icons } from '@/components/ui/icons';
import { useApp } from '@/app/context/ctx-app';

import { WalletNew } from '@dapps-frontend/ui'

export const Header = () => {
  const { isSettled } = useApp();
  const { isAdmin } = useGame();
  const { account } = useAccount();

  return (
    <header className="container flex justify-between items-center py-7.5">
      <Link to="/" className="inline-flex text-white transition-colors hover:text-opacity-70">
        <Icons.logo className="h-15" />
      </Link>

      {account && isSettled && (
        <div className="flex space-x-4 ml-auto">
          {isAdmin && <HeaderAdmin />}
        </div>
      )}

      <div className="ml-4 text-black">
        <WalletNew />
      </div>

    </header>
  );
};
