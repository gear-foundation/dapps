import styles from './Heading.module.scss';
import { cva, VariantProps } from 'class-variance-authority';
import { BaseHTMLAttributes } from 'react';

export const headingVariants = cva('', {
  variants: {
    size: {
      xs: styles.xs,
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
    weight: 'medium',
  },
});

export interface HeadingProps extends BaseHTMLAttributes<HTMLHeadingElement>, VariantProps<typeof headingVariants> {}

export function Heading({ children, className, size, weight, ...props }: HeadingProps) {
  return (
    <h2 className={headingVariants({ size, weight, className })} {...props}>
      {children}
    </h2>
  );
}
