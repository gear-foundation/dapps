import clsx from 'clsx';

import styles from './container.module.scss';

type ContainerProps = BaseComponentProps & {};

// ! TODO:
// @deprecated
export function Container({ children, className }: ContainerProps) {
  return <div className={clsx(styles.container, className)}>{children}</div>;
}
