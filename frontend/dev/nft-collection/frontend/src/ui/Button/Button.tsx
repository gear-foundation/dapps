import { cx } from '@/utils';
import styles from './Button.module.scss';
import { ButtonProps } from './Button.interfaces';

function Button({ variant, icon, label = '', type = 'button', size = 'medium', className, ...props }: ButtonProps) {
  return (
    <button
      className={cx(
        styles.button,
        styles[variant !== 'icon' ? `size-${size}` : ''],
        styles[`variant-${variant}`],
        className || '',
      )}
      type={type}
      {...props}>
      {icon && <img src={icon} alt={label} />}
      {label}
    </button>
  );
}

export { Button };
