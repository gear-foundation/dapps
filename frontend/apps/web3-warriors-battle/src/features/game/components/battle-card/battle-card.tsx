import { VariantProps } from 'class-variance-authority';
import clsx from 'clsx';

import { PlayerSettings } from '@/app/utils';
import { Text } from '@/components';

import { AttackIcon, CupStarIcon, DefenceIcon, DodgeIcon, HealthIcon } from '../../assets/images';
import { Avatar } from '../avatar';
import { CharacterView } from '../character/character';

import styles from './battle-card.module.scss';
import { battleCardVariants } from './battle-card.variants';

type BattleCardProps = PlayerSettings &
  VariantProps<typeof battleCardVariants> & {
    name: string;
    winsCount?: number;
    characterView: CharacterView;
  };

const BattleCard = ({ align, name, attack, health, defence, dodge, winsCount = 0, characterView }: BattleCardProps) => {
  return (
    <div className={battleCardVariants({ className: styles.wrapper, align })}>
      <Avatar size="sm" {...characterView} />
      <div className={styles.info}>
        <div className={styles.header}>
          <Text>{name}</Text>
          <div className={styles.winsCount}>
            <CupStarIcon />
            <Text weight="bold" size="xs">
              {winsCount}
            </Text>
          </div>
        </div>
        <div className={styles.stats}>
          <div className={styles.stat}>
            <HealthIcon />
            <Text size="xs" weight="bold">
              {health}
            </Text>
          </div>
          <div className={clsx(styles.stat, styles.attackStat)}>
            <AttackIcon className={clsx(styles.icon, styles.attack)} />
            <Text size="xs" weight="bold">
              {attack}
            </Text>
          </div>
          <div className={styles.stat}>
            <DefenceIcon className={clsx(styles.icon, styles.reflect)} />
            <Text size="xs" weight="bold">
              {defence}%
            </Text>
          </div>
          <div className={clsx(styles.stat, styles.dodgeStat)}>
            <DodgeIcon className={clsx(styles.icon, styles.dodge)} />
            <Text size="xs" weight="bold">
              {dodge}%
            </Text>
          </div>
        </div>
      </div>
    </div>
  );
};

export { BattleCard };
