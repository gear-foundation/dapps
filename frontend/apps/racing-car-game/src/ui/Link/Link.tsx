import { Link as ReactRputerLink } from 'react-router-dom';

import { LinkProps } from './Link.interfaces';

function Link({ to, children, ...props }: LinkProps) {
  return (
    <ReactRputerLink {...props} to={to}>
      {children}
    </ReactRputerLink>
  );
}

export { Link };
