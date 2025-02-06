import { Link } from 'react-router-dom';

import SVG from '@/assets/images/logo.svg?react';

function Logo() {
  return (
    <Link to="/">
      <SVG />
    </Link>
  );
}

export { Logo };
