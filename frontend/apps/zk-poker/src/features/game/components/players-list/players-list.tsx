import { CrossBoldIcon, Exit, PlusIcon } from '@/assets/images';
import { Avatar, Button } from '@/components';

import styles from './players-list.module.scss';

type Player = {
  avatar?: string;
  name: string;
  isHost?: boolean;
};

type Props = {
  seats: number[];
  players: Player[];
  buyIn: number;
  isAdmin: boolean;
  isSpectator: boolean;
};

const PlayersList = ({ seats, players, buyIn, isAdmin, isSpectator }: Props) => {
  const ExitButton = () => {
    return (
      <Button color="contrast" rounded size="x-small" onClick={() => {}}>
        <Exit />
      </Button>
    );
  };

  const RemovePlayerButton = () => {
    return (
      <Button color="danger" rounded size="x-small" onClick={() => {}}>
        <CrossBoldIcon />
      </Button>
    );
  };

  const AddPlayerButton = () => {
    return (
      <Button color="contrast" rounded size="x-small" onClick={() => {}}>
        <PlusIcon />
      </Button>
    );
  };

  return seats.map((seat) => {
    const player = players[seat];
    const { avatar, name, isHost } = player || {};
    const playerName = player ? `${name} ${isHost ? '(you)' : ''}` : 'Seat Available';
    const points = buyIn.toLocaleString('en-US').replace(',', ' ');

    return (
      <div className={styles.player} key={seat}>
        <div className={styles.playerInfo}>
          <Avatar avatar={avatar} isEmpty={!player} />
          <div className={styles.playerDetails}>
            <span className={styles.playerName}>{playerName}</span>
            <span className={styles.playerDescription}>
              {player ? `${points} PTS` : 'Waiting for a player to join.'}
            </span>
          </div>
        </div>

        <div className={styles.playerActions}>
          {isHost && isAdmin && <ExitButton />}
          {isSpectator && !player && <AddPlayerButton />}
          {isAdmin && !!player && !isHost && <RemovePlayerButton />}
        </div>
      </div>
    );
  });
};

export { PlayersList };
