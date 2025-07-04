import { cva } from 'class-variance-authority';

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
