import { Link, useLocation } from 'react-router-dom';
import { Icon } from 'components/ui/icon';

export const Logo = () => {
  const { pathname } = useLocation();

  return (
    <>
      {pathname !== '/' ? (
        <Link to="/" className="inline-flex text-dark-500 transition-colors hover:text-opacity-70">
          <Icon name="logo-game" width={180} height={44} className="h-10" />
        </Link>
      ) : (
        <span className="inline-flex items-end gap-3">
          <Icon name="train" width={43} height={35} className="w-auto h-9" />
          <Icon name="logo-game" width={140} height={21} className="w-auto h-5 mb-1" />
        </span>
      )}
    </>
  );
};
