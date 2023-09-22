import { cx } from '@/utils';
import styles from './Loader.module.scss';
import { LoaderProps } from './Loader.interface';

function Loader({ wholeScreen }: LoaderProps) {
  return (
    <div className={cx(wholeScreen ? styles.cover : styles.container)}>
      <div className={cx(styles['lds-dual-ring'])} />
    </div>
  );
}

export { Loader };
