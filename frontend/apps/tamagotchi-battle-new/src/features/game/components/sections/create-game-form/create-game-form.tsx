import { useEffect } from 'react';
import { useAccount, useApi, useBalanceFormat } from '@gear-js/react-hooks';
import { useAtomValue } from 'jotai';
import { useNavigate } from 'react-router-dom';
import { Input, Button } from '@gear-js/vara-ui';
import { isNotEmpty, useForm } from '@mantine/form';
import { VaraIcon } from '@/components/layout';
import { usePending } from '@/features/game/hooks';
import { Background } from '../../background';
import { Card } from '@/components';
import { characterAtom } from '@/features/game/store';
import { useCreateNewBattleMessage } from '@/app/utils';
import { ROUTES } from '@/app/consts';
import styles from './create-game-form.module.scss';

type CreateGameFormValues = {
  fee: number;
  name: string;
  tournamentName: string;
};

function CreateGameForm() {
  const navigate = useNavigate();
  const { account } = useAccount();
  const { api } = useApi();
  const { getFormattedBalanceValue } = useBalanceFormat();

  const character = useAtomValue(characterAtom);

  useEffect(() => {
    if (!character) {
      navigate(-1);
    }
  }, [character, navigate]);

  const { createNewBattleMessage } = useCreateNewBattleMessage();
  const { pending, setPending } = usePending();
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

  const { getInputProps: getCreateInputProps, onSubmit: onCreateSubmit } = createForm;

  const handleCreateSession = async (values: CreateGameFormValues) => {
    if (!account?.decodedAddress || !character) {
      return;
    }
    const { appearance, attack, defence, dodge, warriorId } = character;
    const { name, tournamentName } = values;
    const fee = BigInt(getChainBalanceValue(values.fee).toFixed());
    setPending(true);
    await createNewBattleMessage(
      { value: fee, name, tournamentName, appearance, attack, defence, dodge, warriorId },
      {
        onSuccess: () => {
          setPending(false);
          navigate(ROUTES.WAITING);
        },
        onError: () => {
          setPending(false);
        },
      },
    );
  };

  return (
    <Background>
      <Card
        title="Create a private game"
        description="Create your own game tournament, invite your friends, and compete for the ultimate reward."
        size="lg"
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
