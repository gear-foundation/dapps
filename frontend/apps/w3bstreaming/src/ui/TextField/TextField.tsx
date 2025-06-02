import { cx } from '@/utils';

import { TextFieldProps } from './TextField.interfaces';
import styles from './TextField.module.scss';

function TextField({ label, size, placeholder, ...props }: TextFieldProps) {
  return (
    <div className={cx(styles['input-container'])}>
      <input
        className={cx(styles.input)}
        type="text"
        id="fname"
        name="fname"
        value=""
        aria-labelledby="label-fname"
        {...props}
      />
      <label className={cx(styles.label)} htmlFor="fname" id="label-fname">
        <div className={cx(styles.text)}>{label}</div>
      </label>
    </div>
  );
}

export { TextField };
