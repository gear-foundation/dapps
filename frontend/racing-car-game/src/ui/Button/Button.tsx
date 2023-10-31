import { cx } from '@/utils';
import styles from './Button.module.scss';
import { ButtonProps } from './Button.interfaces';

function Button({
  variant,
  icon,
  label = '',
  type = 'button',
  size = 'medium',
  className,
  isLoading,
  disabled,
  ...props
}: ButtonProps) {
  return (
    <button
      className={cx(
        styles.button,
        styles[variant !== 'icon' ? `size-${size}` : ''],
        styles[`variant-${variant}`],
        className || '',
      )}
      type={type}
      disabled={disabled || isLoading}
      {...props}>
      {isLoading && (
        <svg
          xmlns="http://www.w3.org/2000/svg"
          className={styles.loader}
          width={20}
          height={20}
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          strokeWidth="2"
          strokeLinecap="round"
          strokeLinejoin="round">
          <path d="M21 12a9 9 0 1 1-6.219-8.56" />
        </svg>
      )}

      {icon && <img src={icon} alt={label} />}
      {label}
    </button>
  );
}

export { Button };
