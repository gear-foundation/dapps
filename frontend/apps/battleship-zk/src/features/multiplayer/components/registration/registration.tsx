import { decodeAddress } from '@gear-js/api';
import { getVaraAddress, useAccount, useAlert, useBalanceFormat } from '@gear-js/react-hooks';
import { Button } from '@gear-js/vara-ui';
import { stringShorten } from '@polkadot/util';
import clsx from 'clsx';
import { useEffect } from 'react';
import { useNavigate } from 'react-router-dom';

import { getErrorMessage } from '@dapps-frontend/ui';

import { ROUTES } from '@/app/consts';
import { copyToClipboard } from '@/app/utils/utils';
import { VaraIcon } from '@/components/layout/vara-svg';
import { Heading } from '@/components/ui/heading';
import { Text } from '@/components/ui/text';
import { GameCancelledModal, Illustration } from '@/features/game/components';
import { usePending } from '@/features/game/hooks';

import FilledCrossSVG from '../../assets/icons/filled-cross.svg?react';
import { useMultiplayerGame } from '../../hooks';
import {
  useEventGameCancelled,
  useEventPlayerJoinedGame,
  useEventPlayerDeleted,
  useEventGameLeft,
} from '../../sails/events';
import { useCancelGameMessage } from '../../sails/messages';
import { useDeleteGameMessage } from '../../sails/messages/use-delete-player-message';

import styles from './Registration.module.scss';

type UserProps = {
  name: string;
  fee: number;
  address: string;
  isPlayer: boolean;
  isPlayerAdmin: boolean;
  onRemovePlayer: (address: string) => void;
};

function User({ name, fee, address, isPlayer, isPlayerAdmin, onRemovePlayer }: UserProps) {
  const { getFormattedBalanceValue } = useBalanceFormat();
  const { pending } = usePending();

  return (
    <div className={clsx(styles.user, isPlayer && styles.userPlayer)}>
      <div className={styles.left}>
        <div className={styles.crossWrapper}>
          {isPlayerAdmin && !isPlayer && (
            <Button color="transparent" onClick={() => onRemovePlayer(address)} disabled={pending}>
              {<FilledCrossSVG />}
            </Button>
          )}
        </div>
        <div className={styles.name}>{name}</div>
      </div>
      <div className={styles.right}>
        <VaraIcon /> <span>{getFormattedBalanceValue(fee).toFixed(2)}</span>
      </div>
    </div>
  );
}

export function Registration() {
  const navigate = useNavigate();
  const alert = useAlert();
  const { cancelGameMessage } = useCancelGameMessage();
  const { deletePlayerMessage } = useDeleteGameMessage();
  const { account } = useAccount();
  const { game, triggerGame, resetGameState } = useMultiplayerGame();
  const { pending, setPending } = usePending();

  useEventPlayerJoinedGame();
  const { isGameCancelled, onGameCancelled } = useEventGameCancelled();
  const { isPlayerDeleted, onPlayerDeleted } = useEventPlayerDeleted();
  const { isGameLeft, onGameLeft } = useEventGameLeft();

  const startGame = () => {
    navigate(ROUTES.GAME);
  };

  const cancelGame = async () => {
    if (!account?.address) {
      return;
    }

    setPending(true);

    try {
      const transaction = await cancelGameMessage();
      const { response } = await transaction.signAndSend();

      await response();
      resetGameState();
    } catch (error) {
      console.error(error);
      alert.error(getErrorMessage(error));
    } finally {
      setPending(false);
    }
  };

  const handleRemovePlayer = async (address: string) => {
    setPending(true);

    try {
      const transaction = await deletePlayerMessage(address);
      const { response } = await transaction.signAndSend();

      await response();
      await triggerGame();
    } catch (error) {
      console.error(error);
      alert.error(getErrorMessage(error));
    } finally {
      setPending(false);
    }
  };

  const handleCopyAddress = (value: string) => {
    void copyToClipboard({ alert, value });
  };

  useEffect(() => {
    if (game) {
      const currentStatus = Object.keys(game.status)?.[0];
      if (!['registration', 'verificationPlacement'].includes(currentStatus)) {
        navigate(ROUTES.GAME);
      }
    }
  }, [game]);

  return (
    <div className={styles.container}>
      {game && (
        <div className={styles.content}>
          <Illustration />
          <div className={styles.header}>
            <Heading className={styles.mainHeading}>Registration...</Heading>
            <div>
              <Text className={clsx(styles.mainText, styles.mainTextGrey)}>
                Players ({game.participants_data.length}/2). Waiting for other players...
              </Text>
              <span className={styles.addressWrapper}>
                <Text className={styles.mainText}>
                  Share the game&apos;s address:{' '}
                  <span className={styles.mainTextAddress}>({stringShorten(getVaraAddress(game.admin), 4)})</span>
                </Text>
                <Button
                  color="transparent"
                  className={styles.copyButton}
                  onClick={() => handleCopyAddress(getVaraAddress(game.admin))}>
                  Copy
                </Button>
              </span>
            </div>
          </div>
          <div className={styles.controlsWrapper}>
            {game.participants_data.map((item) => (
              <User
                key={item[1].name}
                name={item[1].name}
                address={item[0]}
                fee={Number(game.bid)}
                isPlayer={decodeAddress(item[0]) === account?.decodedAddress}
                isPlayerAdmin={account?.decodedAddress === game.admin}
                onRemovePlayer={handleRemovePlayer}
              />
            ))}
          </div>
          <div className={styles.buttons}>
            {game.admin === account?.decodedAddress && (
              <Button className={styles.cancelGameButton} onClick={cancelGame} disabled={pending}>
                Cancel game
              </Button>
            )}
            <Button disabled={game.participants_data.length < 2} onClick={startGame} isLoading={pending}>
              Start game
            </Button>
          </div>
        </div>
      )}

      {isPlayerDeleted && (
        <GameCancelledModal
          text={'You have been removed from the game by an administrator.'}
          onClose={onPlayerDeleted}
        />
      )}
      {isGameCancelled && (
        <GameCancelledModal text={'The game was terminated by the administrator.'} onClose={onGameCancelled} />
      )}
      {isGameLeft && <GameCancelledModal text={'Your opponent has left the game.'} onClose={onGameLeft} />}
    </div>
  );
}
