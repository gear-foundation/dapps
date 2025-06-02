import { cx } from '@/utils';

import { LoaderProps } from './Loader.interface';
import styles from './Loader.module.scss';

function Loader({ wholeScreen }: LoaderProps) {
  return (
    <div className={cx(wholeScreen ? styles.cover : styles.container)}>
      <div className={cx(styles['lds-ripple'])}>
        <div />
        <div />
      </div>
    </div>
  );
}

export { Loader };
