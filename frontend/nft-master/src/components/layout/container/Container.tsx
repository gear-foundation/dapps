import clsx from 'clsx';
import { ReactNode } from 'react';
import styles from './Container.module.scss';

type Props = {
  children: ReactNode;
  className?: string;
};

function Container({ children, className }: Props) {
  return <div className={clsx(styles.container, className)}>{children}</div>;
}

export { Container };
