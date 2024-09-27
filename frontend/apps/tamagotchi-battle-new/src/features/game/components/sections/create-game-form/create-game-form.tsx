import { useAccount, useAlert, useApi, useBalanceFormat } from '@gear-js/react-hooks';
import { useSetAtom } from 'jotai';
import { useNavigate } from 'react-router-dom';
import { Input, Button } from '@gear-js/vara-ui';
import { isNotEmpty, useForm } from '@mantine/form';
import { VaraIcon } from '@/components/layout';
// import { usePending } from '@/features/game/hooks';
// import { useMultiplayerGame } from '../../hooks';
// import { useCreateGameMessage } from '../../sails/messages';
import styles from './CreateGameForm.module.scss';
import { Background } from '../../background';
import { Card } from '@/components';
import { gameStatusAtom } from '@/features/game/store';

type CreateGameFormValues = {
  fee: number;
  name: string;
  tournamentName: string;
};

type Props = {};

function CreateGameForm({}: Props) {
  const navigate = useNavigate();
  const { account } = useAccount();
  const { api } = useApi();
  const alert = useAlert();
  const setGameStatus = useSetAtom(gameStatusAtom);
  const { getFormattedBalanceValue } = useBalanceFormat();

  const pending = false;
  // const { createGameMessage } = useCreateGameMessage();
  // const { triggerGame } = useMultiplayerGame();
  // const { pending, setPending } = usePending();
  const existentialDeposit = Number(getFormattedBalanceValue(api?.existentialDeposit.toNumber() || 0).toFixed());
  const { getChainBalanceValue } = useBalanceFormat();

  const createForm = useForm<CreateGameFormValues>({
    initialValues: {
      fee: 0,
      name: '',
      tournamentName: '',
    },
    validate: {
      fee: (value) =>
        Number(value) < existentialDeposit + 5 && Number(value) !== 0
          ? `value must be more than ${existentialDeposit + 5} or 0`
          : null,
      name: isNotEmpty(`Name shouldn't be empty`),
      tournamentName: isNotEmpty(`Tournament name shouldn't be empty`),
    },
  });

  const { errors: createErrors, getInputProps: getCreateInputProps, onSubmit: onCreateSubmit } = createForm;

  const handleCreateSession = async (values: CreateGameFormValues) => {
    // if (!account?.decodedAddress) {
    //   return;
    // }
    // try {
    //   setPending(true);
    //   const transaction = await createGameMessage(values.name, BigInt(getChainBalanceValue(values.fee).toFixed()));
    //   const { response } = await transaction.signAndSend();
    //   await response();
    //   await triggerGame();
    // } catch (err) {
    //   const { message, docs } = err as Error & { docs: string };
    //   const errorText = message || docs || 'Create game error';
    //   alert.error(errorText);
    //   console.log(err);
    // } finally {
    //   setPending(false);
    // }
  };

  return (
    <Background>
      <Card
        title="Create a private game"
        description="Create your own game tournament, invite your friends, and compete for the ultimate reward."
        className={styles.card}>
        <form className={styles.form} id="create_game_form" onSubmit={onCreateSubmit(handleCreateSession)}>
          <div className={styles.formRow}>
            <Input
              label="Enter tournament name:"
              placeholder="Tournament name"
              maxLength={20}
              disabled={pending}
              {...getCreateInputProps('tournamentName')}
            />
            <Input
              label="Specify entry fee"
              type="number"
              icon={VaraIcon}
              disabled={pending}
              {...getCreateInputProps('fee')}
            />
          </div>
          <Input
            label="Enter your name"
            placeholder="Your name"
            maxLength={20}
            disabled={pending}
            {...getCreateInputProps('name')}
          />
        </form>

        <div className={styles.buttons}>
          <Button
            text="Back"
            color="grey"
            isLoading={pending}
            size="small"
            className={styles.button}
            onClick={() => navigate(-1)}
          />
          <Button
            type="submit"
            form="create_game_form"
            text="Create game"
            size="small"
            isLoading={pending}
            className={styles.button}
          />
        </div>
      </Card>
    </Background>
  );
}

export { CreateGameForm };
