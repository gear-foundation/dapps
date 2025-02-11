import { decodeAddress, HexString } from '@gear-js/api';
import { useAccount, useBalanceFormat } from '@gear-js/react-hooks';
import { Button } from '@gear-js/vara-ui';
import { isNotEmpty, useForm } from '@mantine/form';
import { useAtom, useSetAtom } from 'jotai';
import { useState } from 'react';

import { CURRENT_GAME_ADMIN_ATOM, CURRENT_STRATEGY_ID_ATOM, IS_LOADING, PLAYER_NAME_ATOM } from '@/atoms';
import { TextField } from '@/components/layout/text-field';

import { GameFoundModal, JoinModalFormValues } from '../../../home/game-found-modal';
import { TextModal } from '../../../home/text-modal';

import styles from './JoinGameForm.module.scss';
import { GameState, useGetGameSessionQuery } from '@/app/utils';
import { getSafeDecodedAddress } from '@/utils';

type Props = {
  onCancel: () => void;
};

type JoinFormValues = {
  address: HexString | undefined;
};

function JoinGameForm({ onCancel }: Props) {
  const { account } = useAccount();
  const { getFormattedBalanceValue } = useBalanceFormat();
  const [foundState, setFoundState] = useState<GameState | null>(null);
  const setCurrentGame = useSetAtom(CURRENT_GAME_ADMIN_ATOM);
  const setCurrentStrategyId = useSetAtom(CURRENT_STRATEGY_ID_ATOM);
  const setPlayerName = useSetAtom(PLAYER_NAME_ATOM);
  const [isLoading] = useAtom(IS_LOADING);
  const [isJoinSessionModalShown, setIsJoinSessionModalShown] = useState<boolean>(false);
  const [foundGame, setFoundGame] = useState<HexString | undefined>(undefined);
  const [gameNotFoundModal, setGameNotFoundModal] = useState<boolean>(false);

  const joinForm = useForm<JoinFormValues>({
    initialValues: {
      address: undefined,
    },
    validate: {
      address: isNotEmpty(`Address shouldn't be empty`),
    },
  });

  const { errors: joinErrors, getInputProps: getJoinInputProps, onSubmit: onJoinSubmit, values } = joinForm;

  const decodedAdminAddress = getSafeDecodedAddress(values.address?.trim());
  const { refetch } = useGetGameSessionQuery(decodedAdminAddress, true);

  const handleCloseFoundModal = () => {
    setIsJoinSessionModalShown(false);
  };

  const handleOpenJoinSessionModal = async () => {
    if (!account?.decodedAddress || !decodedAdminAddress) {
      setGameNotFoundModal(true);
      return;
    }

    try {
      const { data: state } = await refetch();
      if (state) {
        setFoundState(state);
        setFoundGame(decodedAdminAddress);
        setIsJoinSessionModalShown(true);
        return;
      }

      setGameNotFoundModal(true);
    } catch (err: any) {
      console.error(err?.message);
      setGameNotFoundModal(true);
    }
  };

  const handleJoinSession = (values: JoinModalFormValues) => {
    if (foundGame) {
      setCurrentGame(foundGame);
      setCurrentStrategyId(decodeAddress(values.strategyId));
      setPlayerName(values.name);
    }
  };

  const handleCloseNotFoundModal = () => {
    setGameNotFoundModal(false);
  };

  return (
    <>
      <form className={styles.form} onSubmit={onJoinSubmit(handleOpenJoinSessionModal)}>
        <div className={styles.input}>
          <TextField
            label="Specify the game admin address:"
            variant="active"
            placeholder="kG25c..."
            disabled={isLoading}
            {...getJoinInputProps('address')}
          />
          <span className={styles.fieldError}>{joinErrors.address}</span>
        </div>
        <div className={styles.buttons}>
          <Button type="submit" text="Continue" disabled={isLoading} className={styles.button} />
          <Button
            type="submit"
            text="Cancel"
            color="dark"
            disabled={isLoading}
            className={styles.button}
            onClick={onCancel}
          />
        </div>
      </form>

      {isJoinSessionModalShown && foundState && (
        <GameFoundModal
          entryFee={foundState.entry_fee ? getFormattedBalanceValue(String(foundState.entry_fee)).toFixed() : 0}
          players={foundState.players.length}
          gasAmount={1.121}
          onSubmit={handleJoinSession}
          onClose={handleCloseFoundModal}
        />
      )}
      {gameNotFoundModal && (
        <TextModal
          heading="Game not found"
          text="Please check the entered address. It&#39;s possible the game has been canceled or does not exist."
          onClose={handleCloseNotFoundModal}
        />
      )}
    </>
  );
}

export { JoinGameForm };
