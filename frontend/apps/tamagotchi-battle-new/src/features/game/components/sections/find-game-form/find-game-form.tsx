import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Button, Input } from '@gear-js/vara-ui';
import { decodeAddress } from '@gear-js/api';
import { useAccount, useBalanceFormat, withoutCommas } from '@gear-js/react-hooks';
import { isNotEmpty, useForm } from '@mantine/form';
import { HexString } from '@gear-js/api';

import { GameFoundModal, JoinModalFormValues } from '../../modals/game-found-modal';
import styles from './find-game-form.module.scss';
import { Card, Modal } from '@/components';
import { BattleState, useBattleQuery, useRegisterMessage } from '@/app/utils';
import { usePending } from '@/features/game/hooks';
import { ROUTES } from '@/app/consts';
import { useAtomValue } from 'jotai';
import { characterAtom } from '@/features/game/store';

type Props = {};

type FindGameFormValues = {
  address: HexString | undefined;
};

function FindGameForm({}: Props) {
  const navigate = useNavigate();
  const { account } = useAccount();
  const { getFormattedBalanceValue } = useBalanceFormat();
  const { registerMessage } = useRegisterMessage();

  const [foundState, setFoundState] = useState<BattleState | null>(null);

  const { pending, setPending } = usePending();
  const [isJoinSessionModalShown, setIsJoinSessionModalShown] = useState<boolean>(false);
  const [gameNotFoundModal, setGameNotFoundModal] = useState<boolean>(false);

  const joinForm = useForm<FindGameFormValues>({
    initialValues: {
      address: undefined,
    },
    validate: {
      address: isNotEmpty(`Address shouldn't be empty`),
    },
  });

  const { errors: joinErrors, getInputProps: getJoinInputProps, onSubmit: onJoinSubmit, values } = joinForm;

  const { refetch } = useBattleQuery(values.address?.length === 49 ? decodeAddress(values.address) : '');
  const character = useAtomValue(characterAtom);

  useEffect(() => {
    if (!character) {
      navigate(ROUTES.HOME);
    }
  }, [character]);

  const handleCloseFoundModal = () => {
    setIsJoinSessionModalShown(false);
  };

  const handleOpenJoinSessionModal = async (values: FindGameFormValues) => {
    if (!account?.decodedAddress || !values.address) {
      return;
    }

    try {
      const response = await refetch();
      console.log('🚀 ~ handleOpenJoinSessionModal ~ response:', response);
      const { data } = response;
      console.log('🚀 ~ handleOpenJoinSessionModal ~ data:', data);

      if (data?.state && 'registration' in data.state) {
        setFoundState(data);
        setIsJoinSessionModalShown(true);
        return;
      }

      setGameNotFoundModal(true);
    } catch (err: any) {
      setGameNotFoundModal(true);
    }
  };

  const handleJoinSession = async (values: JoinModalFormValues) => {
    if (foundState && account && character) {
      setPending(true);
      const gameId = decodeAddress(foundState.admin);

      const { appearance, attack, defence, dodge, warriorId } = character;
      const { name } = values;
      registerMessage(
        { value: BigInt(foundState.bid), name, appearance, attack, defence, dodge, warriorId, gameId },
        {
          onSuccess: () => {
            setPending(false);
            navigate(ROUTES.WAITING);
          },
          onError: () => setPending(false),
        },
      );
    }
  };

  const handleCloseNotFoundModal = () => {
    setGameNotFoundModal(false);
  };

  return (
    <Card
      title="Find a private game"
      description="To find the game, you need to enter the administrator&#39;s address."
      size="lg"
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
