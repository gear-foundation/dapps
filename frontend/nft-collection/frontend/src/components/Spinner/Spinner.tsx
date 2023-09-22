import { cx } from '@/utils';
import styles from './Spinner.module.scss';

function Spinner() {
  return <div className={cx(styles['lds-dual-ring'])} />;
}

export { Spinner };
