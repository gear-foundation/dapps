import clsx from 'clsx';
import { BaseComponentProps } from '@/app/types';
import { ArrowRightIcon } from '@/features/game/assets/images';
import { Text } from '../text';
import styles from './card-button.module.scss';

type CardButtonProps = BaseComponentProps & {
  onClick: React.MouseEventHandler<HTMLDivElement>;
  title: string;
  subTitle: string;
  icon: React.ReactNode;
};

export function CardButton({ onClick, icon, title, className, subTitle }: CardButtonProps) {
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
          {subTitle}
        </Text>
      </div>
      <ArrowRightIcon />
    </div>
  );
}
