import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Button, Input } from '@gear-js/vara-ui';
import { decodeAddress } from '@gear-js/api';
import { useAccount, useAlert, useBalanceFormat, withoutCommas } from '@gear-js/react-hooks';
import { isNotEmpty, useForm } from '@mantine/form';
import { HexString } from '@gear-js/api';

import { GameFoundModal, JoinModalFormValues } from '../../modals/game-found-modal';
import styles from './find-game-form.module.scss';
import { Card, Modal } from '@/components';

type Props = {};

type FindGameFormValues = {
  address: HexString | undefined;
};

function FindGameForm({}: Props) {
  const navigate = useNavigate();
  const { account } = useAccount();
  const { getFormattedBalanceValue } = useBalanceFormat();
  // const { joinGameMessage } = useJoinGameMessage();
  const alert = useAlert();
  // const gameQuery = useMultiGameQuery();
  // const [foundState, setFoundState] = useState<MultipleGameState | null>(null);

  const foundState = { bid: '123' };
  // const { pending, setPending } = usePending();
  const [isJoinSessionModalShown, setIsJoinSessionModalShown] = useState<boolean>(false);
  console.log('🚀 ~ FindGameForm ~ isJoinSessionModalShown:', isJoinSessionModalShown);
  const [foundGame, setFoundGame] = useState<HexString | undefined>(undefined);
  const [gameNotFoundModal, setGameNotFoundModal] = useState<boolean>(false);

  const pending = false;

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

  const handleOpenJoinSessionModal = async (values: FindGameFormValues) => {
    if (!account?.decodedAddress || !values.address) {
      return;
    }

    try {
      const decodedAdminAddress = decodeAddress(values.address);

      // const state = await gameQuery(decodedAdminAddress.trim());

      // if (state?.status && Object.keys(state.status)[0] === 'registration') {
      //   setFoundState(state);
      //   setFoundGame(decodedAdminAddress);
      setIsJoinSessionModalShown(true);
      return;
      // }

      setGameNotFoundModal(true);
    } catch (err: any) {
      setGameNotFoundModal(true);
    }
  };

  const handleJoinSession = async (values: JoinModalFormValues) => {
    // if (foundGame && foundState && account) {
    //   setPending(true);
    //   try {
    //     // const transaction = await joinGameMessage(foundGame, values.name, BigInt(foundState.bid));
    //     // const { response } = await transaction.signAndSend();
    //     // await response();
    //     // await triggerGame();
    //   } catch (err) {
    //     console.log(err);
    //     const { message, docs } = err as Error & { docs: string };
    //     const errorText = message || docs || 'Create game error';
    //     alert.error(errorText);
    //   } finally {
    //     setPending(false);
    //   }
    // }
  };

  const handleCloseNotFoundModal = () => {
    setGameNotFoundModal(false);
  };

  return (
    <Card
      title="Find a private game"
      description="To find the game, you need to enter the administrator&#39;s address."
      className={styles.card}>
      <form className={styles.form} onSubmit={onJoinSubmit(handleOpenJoinSessionModal)}>
        <div className={styles.input}>
          <Input
            label="Specify the game admin address:"
            placeholder="kG25c..."
            disabled={pending}
            {...getJoinInputProps('address')}
          />
          <span className={styles.fieldError}>{joinErrors.address}</span>
        </div>
        <div className={styles.buttons}>
          <Button type="submit" text="Back" color="grey" disabled={pending} onClick={() => navigate(-1)} />
          <Button type="submit" text="Continue" disabled={pending} />
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
        <Modal
          title="Game not found"
          description="Please check the entered address. It&#39;s possible the game has been canceled or does not exist."
          className={styles.gameNotFoundModal}
          onClose={handleCloseNotFoundModal}
          buttons={<Button color="grey" text="OK" onClick={handleCloseNotFoundModal} />}
        />
      )}
    </Card>
  );
}

export { FindGameForm };
