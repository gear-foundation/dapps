import { HexString } from '@gear-js/api';
import { useAccount } from '@gear-js/react-hooks';

import { CrossBoldIcon, Exit } from '@/assets/images';
import { Avatar, Button } from '@/components';

import { useCancelRegistrationMessage, useDeletePlayerMessage, useKillMessage } from '../../sails';

import styles from './players-list.module.scss';

type Player = {
  avatar?: string;
  name: string;
  isHost?: boolean;
  address: HexString;
  balance: number;
};

type Props = {
  seats: number[];
  players: Player[];
  isAdmin: boolean;
};

const PlayersList = ({ seats, players, isAdmin }: Props) => {
  const { account } = useAccount();
  const { killMessage, isPending: isKillPending } = useKillMessage();
  const { deletePlayerMessage, isPending: isDeletePending } = useDeletePlayerMessage();
  const { cancelRegistrationMessage, isPending: isCancelRegistrationPending } = useCancelRegistrationMessage();

  const ExitButton = () => {
    return (
      <Button
        color="contrast"
        rounded
        size="x-small"
        onClick={() => cancelRegistrationMessage()}
        disabled={isCancelRegistrationPending}>
        <Exit />
      </Button>
    );
  };

  const KillButton = () => {
    return (
      <Button color="contrast" rounded size="x-small" onClick={() => killMessage()} disabled={isKillPending}>
        <Exit />
      </Button>
    );
  };

  const RemovePlayerButton = ({ address }: { address: HexString }) => {
    return (
      <Button
        color="danger"
        rounded
        size="x-small"
        onClick={() => deletePlayerMessage(address)}
        disabled={isDeletePending}>
        <CrossBoldIcon />
      </Button>
    );
  };

  return seats.map((seat) => {
    const player = players[seat];
    const isMe = account?.decodedAddress === player?.address;
    const { address, name, balance } = player || {};
    const playerName = player ? `${name} ${isMe ? '(you)' : ''}` : 'Seat Available';
    const points = balance?.toLocaleString('en-US').replace(',', ' ');

    return (
      <div className={styles.player} key={seat}>
        <div className={styles.playerInfo}>
          <Avatar address={address} isEmpty={!player} />
          <div className={styles.playerDetails}>
            <span className={styles.playerName}>{playerName}</span>
            <span className={styles.playerDescription}>
              {player ? `${points} PTS` : 'Waiting for a player to join.'}
            </span>
          </div>
        </div>

        <div className={styles.playerActions}>
          {isMe && !isAdmin && <ExitButton />}
          {isMe && isAdmin && <KillButton />}
          {!isMe && isAdmin && !!player && <RemovePlayerButton address={player.address} />}
        </div>
      </div>
    );
  });
};

export { PlayersList };
