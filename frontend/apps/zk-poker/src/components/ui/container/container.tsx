import clsx from 'clsx';

import styles from './container.module.scss';

type ContainerProps = BaseComponentProps & {};

export function Container({ children, className }: ContainerProps) {
  return <div className={clsx(styles.container, className)}>{children}</div>;
}
