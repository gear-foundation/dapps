import styles from './Loader.module.scss';

import { cx } from '@/utils';

export function Loader() {
  return <div className={cx(styles.loader)} />;
}
