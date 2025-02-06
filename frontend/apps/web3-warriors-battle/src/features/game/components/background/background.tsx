import clsx from 'clsx';
import { PropsWithChildren } from 'react';

import { backgroundSvg } from '../../assets/images';

import styles from './background.module.scss';

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
