import { cva } from 'class-variance-authority';

import styles from './battle-card.module.scss';

export const battleCardVariants = cva('', {
  variants: { align: { left: styles.left, right: styles.right } },
  defaultVariants: { align: 'left' },
});
