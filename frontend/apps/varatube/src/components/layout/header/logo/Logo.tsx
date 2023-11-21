import { Link } from 'react-router-dom';
import logo from 'assets/images/logo.png';

function Logo() {
  return (
    <Link to="/">
      <img src={logo} alt="" style={{ maxWidth: '150px' }} />
    </Link>
  );
}

export { Logo };
