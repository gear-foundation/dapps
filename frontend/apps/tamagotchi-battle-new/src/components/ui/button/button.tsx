import { cva, type VariantProps } from 'class-variance-authority';
import { ButtonHTMLAttributes, forwardRef } from 'react';
import styles from './buttons.module.scss';

export const buttonVariants = cva('', {
  variants: {
    variant: {
      primary: styles.primary,
      white: styles.white,
      black: styles.black,
      outline: styles.outline,
      text: styles.text,
    },
    size: {
      sm: styles.sm,
      md: styles.md,
    },
    width: {
      normal: '',
      full: styles.block,
    },
    state: {
      normal: '',
      loading: styles.loading,
    },
  },
  // compoundVariants: [{ variant: 'primary', size: 'medium', className: styles.primaryMedium }],
  defaultVariants: {
    variant: 'primary',
    size: 'md',
    state: 'normal',
    width: 'normal',
  },
});

export interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement>, VariantProps<typeof buttonVariants> {
  isLoading?: boolean;
}

export const Button = forwardRef<HTMLButtonElement, ButtonProps>(
  ({ children, className, variant, size, state, isLoading, width, disabled, ...props }, ref) => {
    return (
      <button
        type="button"
        className={buttonVariants({
          variant,
          size,
          state: isLoading ? 'loading' : 'normal',
          width,
          className,
        })}
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
  },
);
