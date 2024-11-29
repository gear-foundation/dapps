import { cva, VariantProps } from 'class-variance-authority';
import { BaseHTMLAttributes } from 'react';
import styles from './text.module.scss';

export const textVariants = cva('', {
  variants: {
    size: {
      sm: styles.sm,
      md: styles.md,
      lg: styles.lg,
      xl: styles.xl,
    },
    weight: {
      normal: styles.normal,
      medium: styles.medium,
      semibold: styles.semibold,
      bold: styles.bold,
    },
  },
  defaultVariants: {
    size: 'md',
    weight: 'normal',
  },
});

export interface TextProps extends BaseHTMLAttributes<HTMLParagraphElement>, VariantProps<typeof textVariants> {}

export function Text({ children, className, size, weight, ...props }: TextProps) {
  return (
    <p className={textVariants({ size, weight, className })} {...props}>
      {children}
    </p>
  );
}
