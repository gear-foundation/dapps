import { cva, VariantProps } from 'class-variance-authority';
import clsx from 'clsx';
import { BaseComponentProps } from '@/app/types';
import { Text } from '../text';
import styles from './card.module.scss';

export const titleVariants = cva('', {
  variants: {
    size: {
      sm: styles.sm,
      md: styles.md,
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
  VariantProps<typeof titleVariants> & {
    title?: string;
    description?: string;
    rightSideSlot?: React.ReactNode;
  };

export function Card({ children, title, align, className, size, description, rightSideSlot }: CardProps) {
  return (
    <div className={clsx(styles.card, size === 'sm' && styles.sm, className)}>
      <div className={styles.leftSide}>
        <div className={styles.header}>
          {title && <h2 className={titleVariants({ align, size })}>{title}</h2>}
          {description && (
            <Text size="sm" className={titleVariants({ className: styles.description, align, size })}>
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
