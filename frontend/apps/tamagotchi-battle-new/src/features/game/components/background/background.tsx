import { PropsWithChildren } from 'react';
import styles from './background.module.scss';
import { backgroundSvg } from '../../assets/images';

type BackgroundProps = PropsWithChildren;

export function Background({ children }: BackgroundProps) {
  return (
    <div className={styles.content}>
      <img src={backgroundSvg} alt="" className={styles.image} />
      {children}
    </div>
  );
}
