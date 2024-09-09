import styles from '../columns.module.scss';
import clsx from 'clsx';
import { BaseComponentProps } from '@/app/types';

export function ColumnsContainer({ children, className }: BaseComponentProps) {
  return <div className={clsx(styles.container, className)}>{children}</div>;
}
