import { cva } from 'class-variance-authority';

import styles from './avatar.module.scss';

export const avatarVariants = cva('', {
  variants: { size: { md: styles.md, sm: styles.sm } },
  defaultVariants: { size: 'md' },
});
