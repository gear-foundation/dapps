import { LogoSVG } from 'components/layout/icons';
import { Link } from 'react-router-dom';

function Logo() {
  return (
    <Link to="/">
      <LogoSVG />
    </Link>
  );
}

export { Logo };
