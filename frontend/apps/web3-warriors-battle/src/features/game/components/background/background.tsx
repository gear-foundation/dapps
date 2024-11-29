import { PropsWithChildren } from 'react';
import styles from './background.module.scss';
import { backgroundSvg } from '../../assets/images';
import clsx from 'clsx';

type BackgroundProps = PropsWithChildren & {
  className?: string;
};

export function Background({ children, className }: BackgroundProps) {
  return (
    <div className={clsx(styles.content, className)}>
      <img src={backgroundSvg} alt="" className={styles.image} />
      {children}
    </div>
  );
}
