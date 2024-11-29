import { cva, VariantProps } from 'class-variance-authority';
import clsx from 'clsx';
import { BaseComponentProps } from '@/app/types';
import { Text } from '../text';
import styles from './card.module.scss';

export const cardVariants = cva('', {
  variants: {
    size: {
      sm: styles.sm,
      md: styles.md,
      lg: styles.lg,
    },
    align: {
      left: styles.left,
      center: styles.center,
    },
  },
  defaultVariants: {
    size: 'md',
    align: 'center',
  },
});

type CardProps = BaseComponentProps &
  VariantProps<typeof cardVariants> & {
    title?: string;
    description?: string;
    rightSideSlot?: React.ReactNode;
  };

export function Card({ children, title, align, className, size, description, rightSideSlot }: CardProps) {
  return (
    <div className={cardVariants({ className: clsx(styles.card, className), size })}>
      <div className={styles.leftSide}>
        <div className={styles.header}>
          {title && <h2 className={cardVariants({ align, size })}>{title}</h2>}
          {description && (
            <Text size="sm" className={cardVariants({ className: styles.description, align, size })}>
              {description}
            </Text>
          )}
        </div>
        {children}
      </div>
      {rightSideSlot && <div className={styles.rightSide}>{rightSideSlot}</div>}
    </div>
  );
}
