import styles from './container.module.scss';
import clsx from 'clsx';

type ContainerProps = BaseComponentProps & {};

export function Container({ children, className }: ContainerProps) {
  return <div className={clsx(styles.container, className)}>{children}</div>;
}
