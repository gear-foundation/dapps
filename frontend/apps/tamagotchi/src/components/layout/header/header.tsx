import { Link } from 'react-router-dom';
import { Wallet } from '@dapps-frontend/ui';
import { SpriteIcon } from '@/components/ui/sprite-icon';

export const Header = () => {
  return (
    <header className="container flex justify-between items-center py-7.5">
      <Link to="/" className="inline-flex text-white transition-colors hover:text-opacity-70">
        <SpriteIcon name="logo" width={180} height={44} className="h-10" />
      </Link>

      <Wallet />
    </header>
  );
};
