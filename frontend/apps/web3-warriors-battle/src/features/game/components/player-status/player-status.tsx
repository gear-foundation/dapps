import { Text } from '@/components';
import { SkullBigIcon } from '../../assets/images';
import clsx from 'clsx';

import styles from './player-status.module.scss';

type PlayerStatusProps = {
  isAlive: boolean;
};

const PlayerStatus = ({ isAlive }: PlayerStatusProps) => {
  return (
    <Text size="xs" weight="semibold" className={clsx(styles.status, isAlive ? styles.alive : styles.defeat)}>
      {!isAlive && <SkullBigIcon />}
      {isAlive ? 'Alive' : 'Defeated'}
    </Text>
  );
};

export { PlayerStatus };
