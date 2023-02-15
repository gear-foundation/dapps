import { Link, useLocation } from 'react-router-dom';
import { Icon } from 'components/ui/icon';

export const Logo = () => {
  const { pathname } = useLocation();

  return (
    <>
      {pathname !== '/' ? (
        <Link to="/" className="inline-flex text-white transition-colors hover:text-opacity-70">
          <img src="images/gasa.png" style={{ width: '100px' }}></img>
        </Link>
      ) : (
        <span className="inline-flex text-white">
          <img src="images/gasa.png" style={{ width: '100px' }}></img>
        </span>
      )}
    </>
  );
};
