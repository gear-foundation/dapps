import cx from 'clsx';
import { ReactNode } from 'react';
import styles from './container.module.css';

type Props = {
  className?: string;
  children: ReactNode;
};

function Container({ className, children }: Props) {
  return <div className={cx(styles.container, className)}>{children}</div>;
}

export { Container };
