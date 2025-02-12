import { cx } from '@/utils';

import { InputProps } from './Input.interfaces';
import styles from './Input.module.scss';

function Input({ size = 'medium', ...props }: InputProps) {
  return (
    <div className={cx(styles.wrapper, styles[`size-${size}`])}>
      <input {...props} className={cx(styles.input)} />
    </div>
  );
}

export { Input };
