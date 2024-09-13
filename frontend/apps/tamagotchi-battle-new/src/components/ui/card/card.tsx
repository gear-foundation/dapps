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
    subTitle?: string;
  };

export function Card({ children, title, align, className, size, subTitle }: CardProps) {
  return (
    <div className={clsx(styles.card, size === 'sm' && styles.sm, className)}>
      <div>
        {title && <h2 className={titleVariants({ align, size })}>{title}</h2>}
        {subTitle && (
          <Text size="sm" className={styles.center}>
            {subTitle}
          </Text>
        )}
      </div>
      {children}
    </div>
  );
}
