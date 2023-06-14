import { ReactNode } from 'react';
import clsx from 'clsx';
import styles from './Container.module.scss';

type Props = {
  children: ReactNode;
  className?: string;
};

function Container({ className, children }: Props) {
  return <div className={clsx(styles.container, className)}>{children}</div>;
}

export { Container };
