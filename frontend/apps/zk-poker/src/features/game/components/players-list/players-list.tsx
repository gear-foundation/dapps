import { HexString } from '@gear-js/api';
import { useAccount } from '@gear-js/react-hooks';

import { CrossBoldIcon, Exit, PlusIcon } from '@/assets/images';
import { Avatar, Button } from '@/components';

import { useCancelRegistrationMessage, useRegisterMessage, useDeletePlayerMessage } from '../../sails';

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
  const isSpectator = !players.some(({ address }) => address === account?.decodedAddress);
  const { cancelRegistrationMessage, isPending: isCancelPending } = useCancelRegistrationMessage();
  const { registerMessage, isPending: isRegisterPending } = useRegisterMessage();
  const { deletePlayerMessage, isPending: isDeletePending } = useDeletePlayerMessage();

  const ExitButton = () => {
    return (
      <Button
        color="contrast"
        rounded
        size="x-small"
        onClick={() => cancelRegistrationMessage()}
        disabled={isCancelPending}>
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

  const RegisterButton = () => {
    return (
      <Button color="contrast" rounded size="x-small" onClick={() => registerMessage()} disabled={isRegisterPending}>
        <PlusIcon />
      </Button>
    );
  };

  return seats.map((seat) => {
    const player = players[seat];
    const isMe = account?.decodedAddress === player?.address;
    const { avatar, name, balance } = player || {};
    const playerName = player ? `${name} ${isMe ? '(you)' : ''}` : 'Seat Available';
    const points = balance?.toLocaleString('en-US').replace(',', ' ');

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
          {isMe && <ExitButton />}
          {isSpectator && !player && <RegisterButton />}
          {isAdmin && !!player && !isMe && <RemovePlayerButton address={player.address} />}
        </div>
      </div>
    );
  });
};

export { PlayersList };
