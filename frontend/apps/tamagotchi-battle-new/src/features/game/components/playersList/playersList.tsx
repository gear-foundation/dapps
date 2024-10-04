import clsx from 'clsx';
import { Button } from '@gear-js/vara-ui';
import { useState } from 'react';
import { BaseComponentProps } from '@/app/types';
import { Text } from '@/components';
import { VaraIcon } from '@/components/layout';
import { Heading } from '@/components/ui/heading';
import { PlayerStatus } from '../player-status/player-status';
import styles from './playersList.module.scss';

type Item = {
  name: string;
  status: 'defeated' | 'alive';
};

type PlayersListProps = BaseComponentProps & {
  items: Item[];
  bid: number;
  tournamentName: string;
};

const PlayersList = ({ items, className, bid, tournamentName, ...restProps }: PlayersListProps) => {
  const [showAll, setShowAll] = useState(false);
  const maxLength = 10;
  const displayedItems = showAll ? items : items.slice(0, maxLength);

  return (
    <div className={clsx(styles.wrapper, className)} {...restProps}>
      <Heading size="md" weight="bold" className={styles.title}>
        {tournamentName}
      </Heading>
      <div className={styles.list}>
        {displayedItems.map(({ name, status }, index) => {
          const isMy = index === 2;

          return (
            <div key={name + index} className={clsx(styles.item, { [styles.my]: isMy })}>
              <Text size="sm" weight="semibold" className={styles.number}>
                {index + 1}
              </Text>
              <Text size="sm" weight="semibold">
                {name}
              </Text>
              <div className={styles.statusWrapper}>
                <PlayerStatus isAlive={status === 'alive'} />

                <VaraIcon className={styles.icon} />
                <Text size="sm" weight="semibold">
                  {bid.toFixed(2)}
                </Text>
              </div>
            </div>
          );
        })}
      </div>
      {items.length > maxLength && !showAll && (
        <Button color="light" text="Show More" onClick={() => setShowAll(true)} />
      )}
    </div>
  );
};

export { PlayersList };
