import clsx from 'clsx';

import { TextFieldProps } from './TextField.interfaces';
import styles from './TextField.module.scss';

function TextField({
  label,
  size,
  placeholder,
  value,
  icon,
  theme = 'dark',
  variant = 'default',
  ...props
}: TextFieldProps) {
  return (
    <div className={clsx(styles.inputContainer, styles[`inputContainer-${theme}`])}>
      {icon && <div className={clsx(styles.iconContainer)}>{icon}</div>}
      <input
        className={clsx(styles.input, styles[`input-${variant}`], styles[`input-${theme}`])}
        type="text"
        id="fname"
        name="fname"
        value={value}
        aria-labelledby="label-fname"
        placeholder={variant === 'active' ? placeholder : undefined}
        {...props}
      />
      <label
        className={clsx(styles.label, styles[`label-${variant}`], styles[`label-${theme}`])}
        htmlFor="fname"
        id="label-fname">
        <div className={clsx(styles.text)}>{label}</div>
      </label>
    </div>
  );
}

export { TextField };
