import { ReactNode } from 'react';
import clsx from 'clsx';
import styles from './Card.module.scss';

type Props = {
  children: ReactNode;
  className?: string;
};

function Card({ children, className }: Props) {
  return <div className={clsx(styles.card, className)}>{children}</div>;
}

export { Card };
