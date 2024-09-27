import clsx from 'clsx';
import { BaseComponentProps } from '@/app/types';
import { ArrowRightIcon } from '@/features/game/assets/images';
import { Text } from '../text';
import styles from './card-button.module.scss';

type CardButtonProps = BaseComponentProps & {
  onClick: React.MouseEventHandler<HTMLDivElement>;
  title: string;
  description: string;
  icon: React.ReactNode;
};

export function CardButton({ onClick, icon, title, className, description }: CardButtonProps) {
  return (
    <div onClick={onClick} className={clsx(styles.card, className)}>
      <div>
        <div className={styles.title}>
          {icon}
          <Text size="lg" weight="semibold">
            {title}
          </Text>
        </div>
        <Text size="sm" className={styles.center}>
          {description}
        </Text>
      </div>
      <ArrowRightIcon />
    </div>
  );
}
