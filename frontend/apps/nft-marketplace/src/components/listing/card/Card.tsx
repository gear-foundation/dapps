import clsx from 'clsx';
import { ReactNode } from 'react';
import styles from './Card.module.scss';

type Props = {
  heading: string;
  text?: string | number;
  children?: ReactNode;
};

function Card({ heading, text, children }: Props) {
  const isDescription = heading === 'Description';
  const isEllipsis = heading === 'Owner' || heading === 'Current Winner';

  const className = clsx(isDescription ? styles.description : styles.text, isEllipsis && styles.ellipsis);

  return (
    <div className={styles.card}>
      <h3 className={styles.heading}>{heading}</h3>
      {children || (text && <p className={className}>{text}</p>)}
    </div>
  );
}

export { Card };
