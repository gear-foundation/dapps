import { cva } from 'class-variance-authority';

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
