import { Link } from 'react-router-dom';

import LogoSVG from '@/assets/images/logo.svg?react';

function Logo() {
  return (
    <Link to="/">
      <LogoSVG />
    </Link>
  );
}

export { Logo };
