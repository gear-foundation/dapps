import { type VariantProps } from 'class-variance-authority';
import { ButtonHTMLAttributes, ForwardedRef, forwardRef } from 'react';

import { buttonVariants } from './button.variants';
import styles from './buttons.module.scss';

export interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement>, VariantProps<typeof buttonVariants> {
  isLoading?: boolean;
}

const ButtonComponent = (
  { children, className, variant, size, state, isLoading, width, disabled, ...props }: ButtonProps,
  ref: ForwardedRef<HTMLButtonElement>,
) => {
  const resolvedState = isLoading ? 'loading' : (state ?? 'normal');

  return (
    <button
      type="button"
      className={buttonVariants({ variant, size, state: resolvedState, width, className })}
      disabled={disabled || isLoading}
      ref={ref}
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
      {children}
    </button>
  );
};

export const Button = forwardRef<HTMLButtonElement, ButtonProps>(ButtonComponent);

Button.displayName = 'Button';
