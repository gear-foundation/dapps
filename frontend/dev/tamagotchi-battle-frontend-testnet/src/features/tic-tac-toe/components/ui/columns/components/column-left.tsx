import clsx from 'clsx';
import styles from '../columns.module.scss';

type ContainerProps = BaseComponentProps & {};

export function ColumnLeft({ children, className }: ContainerProps) {
  return <div className={clsx(styles.container__left, className)}>{children}</div>;
}
