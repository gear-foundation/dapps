/* eslint-disable jsx-a11y/control-has-associated-label */
/* eslint-disable jsx-a11y/label-has-associated-control */
import styles from './TextField.module.scss';
import { TextFieldProps } from './TextField.interfaces';
import clsx from 'clsx';

function TextField({
  label,
  size,
  placeholder,
  value,
  icon,
  theme = 'default',
  variant = 'default',
  ...props
}: TextFieldProps) {
  return (
    <div className={clsx(styles.inputContainer, theme === 'dark' && styles.darkInputContainer)}>
      {icon && <div className={clsx(styles.iconContainer)}>{icon}</div>}
      <input
        className={clsx(
          styles.input,
          variant === 'default' && styles.defaultInput,
          variant === 'active' && styles.activeInput,
          theme === 'dark' && styles.darkInputColors,
        )}
        type="text"
        id="fname"
        name="fname"
        value={value}
        aria-labelledby="label-fname"
        placeholder={variant === 'active' ? placeholder : undefined}
        {...props}
      />
      <label
        className={clsx(
          variant === 'default' && styles.label,
          variant === 'active' && styles.activeLabel,
          theme === 'dark' && styles.darkLabel,
        )}
        htmlFor="fname"
        id="label-fname">
        <div className={clsx(styles.text)}>{label}</div>
      </label>
    </div>
  );
}

export { TextField };
