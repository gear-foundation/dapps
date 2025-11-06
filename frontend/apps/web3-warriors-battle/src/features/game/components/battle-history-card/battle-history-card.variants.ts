import { cva } from 'class-variance-authority';

import styles from './battle-history-card.module.scss';

export const battleHistoryCardVariants = cva('', {
  variants: { align: { left: styles.left, right: styles.right } },
  defaultVariants: { align: 'left' },
});
