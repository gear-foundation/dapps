import { cx } from '@/utils';
import styles from './Loader.module.scss';

export function Loader() {
  return <div className={cx(styles.loader)} />;
}
