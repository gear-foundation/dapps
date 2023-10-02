import { WelcomeProps } from './Welcome.interface';
import styles from './Welcome.module.scss';
import { cx } from '@/utils';

function Welcome({ children }: WelcomeProps) {
  return (
    <div className={cx(styles.content)}>
      <h1 className={cx(styles['main-title'], styles['main-title-with-gradient'])}>NFT Collection</h1>
      <p className={cx(styles['main-description'])}>NFT Collection</p>

      {children}
    </div>
  );
}

export { Welcome };
