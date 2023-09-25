import { cva, type VariantProps } from 'class-variance-authority';
import { ButtonHTMLAttributes } from 'react';
import { Loader2 } from 'lucide-react';
import styles from './buttons.module.scss';

export const buttonVariants = cva(styles.base, {
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
    return (
        <button
            type="button"
            className={buttonVariants({ variant, size, state: isLoading ? 'loading' : 'normal', width, className })}
            disabled={disabled || isLoading}
            // eslint-disable-next-line react/jsx-props-no-spreading
            {...props}>
            {isLoading && <Loader2 width={20} height={20} className={styles.loader} />}
            {children}
        </button>
    );
}