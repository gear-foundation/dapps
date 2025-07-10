import { cva } from 'class-variance-authority';

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
