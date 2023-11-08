import { Link as ReactRputerLink } from 'react-router-dom';
import { LinkProps } from './Link.interfaces';

function Link({ to, children }: LinkProps) {
  return <ReactRputerLink to={to}>{children}</ReactRputerLink>;
}

export { Link };
