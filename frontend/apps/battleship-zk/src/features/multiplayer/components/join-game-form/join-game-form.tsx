import { useState } from 'react';
import { Button } from '@gear-js/vara-ui';
import { decodeAddress } from '@gear-js/api';
import { useAccount, useAlert, useBalanceFormat, withoutCommas } from '@gear-js/react-hooks';
import { TextField } from '@/components/layout/text-field';
import { isNotEmpty, useForm } from '@mantine/form';
import { HexString } from '@gear-js/api';
import { GameFoundModal, JoinModalFormValues } from '../game-found-modal';
import { TextModal } from '@/components/layout/text-modal';
import { Text } from '@/components/ui/text';
import { Heading } from '@/components/ui/heading';
import { usePending } from '@/features/game/hooks';
import { MultipleGameState } from '@/app/utils/sails/lib/lib';
import { useMultiplayerGame } from '../../hooks/use-multiplayer-game';
import { useJoinGameMessage } from '../../sails/messages';
import { useMultiGameQuery } from '../../sails/queries';
import styles from './JoinGameForm.module.scss';

type Props = {
  onCancel: () => void;
};

type JoinFormValues = {
  address: HexString | undefined;
};

function JoinGameForm({ onCancel }: Props) {
  const { account } = useAccount();
  const { getFormattedBalanceValue } = useBalanceFormat();
  const { triggerGame } = useMultiplayerGame();
  const { joinGameMessage } = useJoinGameMessage();
  const alert = useAlert();
  const gameQuery = useMultiGameQuery();
  const [foundState, setFoundState] = useState<MultipleGameState | null>(null);
  const { pending, setPending } = usePending();
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

    try {
      const decodedAdminAddress = decodeAddress(values.address);

      const state = await gameQuery(decodedAdminAddress.trim());

      if (state?.status && Object.keys(state.status)[0] === 'registration') {
        setFoundState(state);
        setFoundGame(decodedAdminAddress);
        setIsJoinSessionModalShown(true);
        return;
      }

      setGameNotFoundModal(true);
    } catch (err: any) {
      setGameNotFoundModal(true);
    }
  };

  const handleJoinSession = async (values: JoinModalFormValues) => {
    if (foundGame && foundState && account) {
      setPending(true);

      try {
        const transaction = await joinGameMessage(foundGame, values.name, BigInt(foundState.bid));
        const { response } = await transaction.signAndSend();

        await response();
        await triggerGame();
      } catch (err) {
        console.log(err);
        const { message, docs } = err as Error & { docs: string };
        const errorText = message || docs || 'Create game error';
        alert.error(errorText);
      } finally {
        setPending(false);
      }
    }
  };

  const handleCloseNotFoundModal = () => {
    setGameNotFoundModal(false);
  };

  return (
    <div className={styles.formWrapper}>
      <div className={styles.header}>
        <Heading className={styles.mainHeading}>Join a peer-to-peer game</Heading>
        <div>
          <Text className={styles.mainText}>Enter the game's address to join the game.</Text>
        </div>
      </div>
      <form className={styles.form} onSubmit={onJoinSubmit(handleOpenJoinSessionModal)}>
        <div className={styles.input}>
          <TextField
            label="Specify the game's address:"
            variant="active"
            placeholder="kG25c..."
            disabled={pending}
            {...getJoinInputProps('address')}
          />
          <span className={styles.fieldError}>{joinErrors.address}</span>
        </div>
        <div className={styles.buttons}>
          <Button type="submit" text="Find game" disabled={pending} className={styles.button} />
          <Button
            type="submit"
            text="Back"
            color="grey"
            disabled={pending}
            className={styles.button}
            onClick={onCancel}
          />
        </div>
      </form>

      {isJoinSessionModalShown && foundState && (
        <GameFoundModal
          entryFee={getFormattedBalanceValue(withoutCommas(String(foundState.bid))).toFixed()}
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
    </div>
  );
}

export { JoinGameForm };
