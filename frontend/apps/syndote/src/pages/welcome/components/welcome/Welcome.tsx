import { ReactElement } from 'react';
import src from '@/assets/images/earth.gif';
import styles from './Welcome.module.scss';

type Props = {
  children: ReactElement;
};

function Welcome({ children }: Props) {
  return (
    <div className={styles.welcomeContainer}>
      <div className={styles.introWrapper}>{children}</div>
      <div className={styles.imageWrapper}>
        <img src={src} alt="earth" className={styles.image} />
      </div>
    </div>
  );
}

export { Welcome };
