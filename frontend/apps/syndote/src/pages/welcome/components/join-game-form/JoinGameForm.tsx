import { useState } from 'react';
import { Button } from '@gear-js/vara-ui';
import { useSetAtom, useAtom } from 'jotai';
import { decodeAddress } from '@gear-js/api';
import { useDnsProgramIds } from '@dapps-frontend/hooks';
import { CURRENT_GAME_ADMIN_ATOM, CURRENT_STRATEGY_ID_ATOM, IS_LOADING, PLAYER_NAME_ATOM } from '@/atoms';
import metaTxt from '@/assets/meta/syndote_meta.txt';
import { useAccount, useApi, useBalanceFormat, withoutCommas } from '@gear-js/react-hooks';
import { TextField } from '@/components/layout/text-field';
import { isNotEmpty, useForm } from '@mantine/form';
import { HexString } from '@gear-js/api';
import { GameFoundModal, JoinModalFormValues } from '../../../home/game-found-modal';
import { useProgramMetadata } from '@/hooks/metadata';
import { TextModal } from '../../../home/text-modal';
import styles from './JoinGameForm.module.scss';
import { GameSessionState, State } from '@/types';

type Props = {
  onCancel: () => void;
};

type JoinFormValues = {
  address: HexString | undefined;
};

function JoinGameForm({ onCancel }: Props) {
  const { account } = useAccount();
  const { api } = useApi();
  const { programId } = useDnsProgramIds();
  const { getFormattedBalanceValue } = useBalanceFormat();
  const [foundState, setFoundState] = useState<State | null>(null);
  const meta = useProgramMetadata(metaTxt);
  const setCurrentGame = useSetAtom(CURRENT_GAME_ADMIN_ATOM);
  const setCurrentStrategyId = useSetAtom(CURRENT_STRATEGY_ID_ATOM);
  const setPlayerName = useSetAtom(PLAYER_NAME_ATOM);
  const [isLoading] = useAtom(IS_LOADING);
  const [isJoinSessionModalShown, setIsJoinSessionModalShown] = useState<boolean>(false);
  const [foundGame, setFoundGame] = useState<HexString | undefined>(undefined);
  const [gameNotFoundModal, setGameNotFoundModal] = useState<boolean>(false);

  const joinForm = useForm({
    initialValues: {
      address: undefined,
    },
    validate: {
      address: isNotEmpty(`Address shouldn't be empty`),
    },
  });

  const { errors: joinErrors, getInputProps: getJoinInputProps, onSubmit: onJoinSubmit } = joinForm;

  const handleCloseFoundModal = () => {
    setIsJoinSessionModalShown(false);
  };

  const handleOpenJoinSessionModal = async (values: JoinFormValues) => {
    if (!account?.decodedAddress || !values.address) {
      return;
    }

    const decodedAdminAddress = decodeAddress(values.address);

    const payload = { GetGameSession: { accountId: decodedAdminAddress.trim() } };

    try {
      const res = await api?.programState.read(
        {
          programId,
          payload,
        },
        meta,
      );

      const state = (await res?.toHuman()) as GameSessionState;
      console.log('STATE');
      console.log(state.GameSession.gameSession);

      if (state.GameSession.gameSession) {
        setFoundState(state.GameSession.gameSession);
        setFoundGame(decodedAdminAddress);
        setIsJoinSessionModalShown(true);
        return;
      }

      setGameNotFoundModal(true);
    } catch (err: any) {
      console.log(err.message);
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

      {isJoinSessionModalShown && (
        <GameFoundModal
          entryFee={getFormattedBalanceValue(withoutCommas(foundState?.entryFee || '')).toFixed()}
          players={foundState?.players.length || 1}
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
