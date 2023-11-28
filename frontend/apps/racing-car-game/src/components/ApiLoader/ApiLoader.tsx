import { cx } from '@/utils';
import styles from './ApiLoader.module.scss';

function ApiLoader() {
  return <p className={cx(styles.loader)}>Initializing API</p>;
}

export { ApiLoader };
