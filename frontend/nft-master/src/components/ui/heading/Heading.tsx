import { cva, type VariantProps } from 'class-variance-authority';
import { BaseHTMLAttributes } from 'react';
import styles from './Heading.module.scss';

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
    // eslint-disable-next-line react/jsx-props-no-spreading
    <h2 className={headingVariants({ size, weight, className })} {...props}>
      {children}
    </h2>
  );
}
