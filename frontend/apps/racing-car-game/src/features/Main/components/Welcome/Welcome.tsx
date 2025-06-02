import carsImgPng from '@/assets/icons/introdution-cars-img.png';
import carsImg from '@/assets/icons/introdution-cars-img.webp';
import { cx } from '@/utils';

import { WelcomeProps } from './Welcome.interface';
import styles from './Welcome.module.scss';

function Welcome({ children }: WelcomeProps) {
  return (
    <div className={cx(styles.content)}>
      <div className={cx(styles.left)}>
        <h1 className={cx(styles['main-title'], styles['main-title-with-gradient'])}>Racing Car Game</h1>
        <p className={cx(styles['main-description'])}>
          A racing car game in which you compete not against a human, but against a smart contract. You will be given
          the choice to either accelerate or shoot at the nearest car to slow it down.
        </p>
        {children}
      </div>
      <div className={cx(styles.right)}>
        <picture>
          <source type="image/webp" srcSet={carsImg} />
          <source type="image/jpeg" srcSet={carsImgPng} />
          <img src={carsImg} alt="test" className={cx(styles['cars-pic'])} />
        </picture>
      </div>
    </div>
  );
}

export { Welcome };
