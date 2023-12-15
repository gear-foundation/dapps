/* eslint-disable jsx-a11y/control-has-associated-label */
/* eslint-disable jsx-a11y/label-has-associated-control */
import { cx } from '@/utils';
import styles from './TextField.module.scss';
import { TextFieldProps } from './TextField.interfaces';

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
