import { ReactElement } from 'react';
import { cx } from 'utils';
import src from 'assets/images/earth.gif';
import { GameIntro } from '../game-intro';
import styles from './Welcome.module.scss';

type Props = {
  children: ReactElement;
};

function Welcome({ children }: Props) {
  return (
    <div className={cx(styles.welcomeContainer)}>
      <div className={cx(styles.introWrapper)}>
        <GameIntro />
        {children}
      </div>
      <div className={cx(styles.imageWrapper)}>
        <img src={src} alt="earth" className={styles.image} />
      </div>
    </div>
  );
}

export { Welcome };
