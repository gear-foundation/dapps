import { type VariantProps } from 'class-variance-authority';
import { Loader2 } from 'lucide-react';
import { ButtonHTMLAttributes } from 'react';

import { buttonVariants } from './button-variants';
import styles from './buttons.module.scss';

export interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement>, VariantProps<typeof buttonVariants> {
  isLoading?: boolean;
}

export function Button({
  children,
  className,
  variant,
  size,
  state,
  isLoading,
  width,
  disabled,
  ...props
}: ButtonProps) {
  const resolvedState = state ?? (isLoading ? 'loading' : 'normal');

  return (
    <button
      type="button"
      className={buttonVariants({
        variant,
        size,
        state: resolvedState,
        width,
        className,
      })}
      disabled={disabled || isLoading}
      {...props}>
      {isLoading && <Loader2 width={20} height={20} className={styles.loader} />}
      {children}
    </button>
  );
}
