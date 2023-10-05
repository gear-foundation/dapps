import { Link } from 'react-router-dom';
import { ReactComponent as GalexSVG } from 'assets/images/logo.svg';
import { ReactComponent as VaraSVG } from 'assets/images/logo-vara.svg';
import { cx } from 'utils';
import styles from '../Header.module.scss';

function Logo() {
  return (
    <Link to="/">
      <VaraSVG className={cx(styles['vara-logo'])} />
      <GalexSVG />
    </Link>
  );
}

export { Logo };
