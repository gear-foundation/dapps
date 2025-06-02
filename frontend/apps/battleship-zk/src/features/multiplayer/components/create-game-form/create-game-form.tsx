import { useAccount, useAlert, useApi, useBalanceFormat } from '@gear-js/react-hooks';
import { Button } from '@gear-js/vara-ui';
import { isNotEmpty, useForm } from '@mantine/form';
import { EzTransactionsSwitch } from 'gear-ez-transactions';

import { getErrorMessage } from '@dapps-frontend/ui';

import { SIGNLESS_ALLOWED_ACTIONS } from '@/app/consts';
import { TextField } from '@/components/layout/text-field';
import { VaraIcon } from '@/components/layout/vara-svg';
import { Heading } from '@/components/ui/heading';
import { Text } from '@/components/ui/text';

import { usePending } from '@/features/game/hooks';

import { useMultiplayerGame } from '../../hooks';
import { useCreateGameMessage } from '../../sails/messages';

import styles from './CreateGameForm.module.scss';

type CreateFormValues = {
  fee: number;
  name: string;
};

type Props = {
  onCancel: () => void;
};

function CreateGameForm({ onCancel }: Props) {
  const { account } = useAccount();
  const { api } = useApi();
  const alert = useAlert();
  const { getFormattedBalanceValue } = useBalanceFormat();
  const { createGameMessage } = useCreateGameMessage();
  const { triggerGame } = useMultiplayerGame();
  const { pending, setPending } = usePending();
  const existentialDeposit = Number(getFormattedBalanceValue(api?.existentialDeposit.toNumber() || 0).toFixed());
  const { getChainBalanceValue } = useBalanceFormat();

  const createForm = useForm({
    initialValues: {
      fee: 0,
      name: '',
    },
    validate: {
      fee: (value) =>
        Number(value) < existentialDeposit + 5 && Number(value) > 0
          ? `value must be more than ${existentialDeposit + 5} or 0`
          : null,
      name: isNotEmpty(`Name shouldn't be empty`),
    },
  });

  const { errors: createErrors, getInputProps: getCreateInputProps, onSubmit: onCreateSubmit } = createForm;

  const handleCreateSession = async (values: CreateFormValues) => {
    if (!account?.decodedAddress) {
      return;
    }

    try {
      setPending(true);

      const transaction = await createGameMessage(values.name, BigInt(getChainBalanceValue(values.fee).toFixed()));
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

  return (
    <div className={styles.formWrapper}>
      <div className={styles.header}>
        <Heading className={styles.mainHeading}>Create a private game</Heading>
        <div>
          <Text className={styles.mainText}>
            Configure the game settings and click 'Create game'. Share the game's address to invite a friend.
          </Text>
        </div>
      </div>
      <form className={styles.form} id="create_game_form" onSubmit={onCreateSubmit(handleCreateSession)}>
        <div className={styles.input}>
          <TextField
            label="Specify entry fee"
            variant="active"
            type="number"
            icon={<VaraIcon />}
            disabled={pending}
            {...getCreateInputProps('fee')}
          />
          <span className={styles.fieldError}>{createErrors.fee}</span>
        </div>
        <div className={styles.input}>
          <TextField
            label="Enter your name"
            variant="active"
            placeholder="Your name"
            maxLength={20}
            disabled={pending}
            {...getCreateInputProps('name')}
          />
          <span className={styles.fieldError}>{createErrors.name}</span>
        </div>
      </form>
      <EzTransactionsSwitch allowedActions={SIGNLESS_ALLOWED_ACTIONS} />
      <div className={styles.buttons}>
        <Button
          type="submit"
          form="create_game_form"
          text="Create game"
          isLoading={pending}
          className={styles.button}
        />
        <Button text="Cancel" color="grey" isLoading={pending} className={styles.button} onClick={onCancel} />
      </div>
    </div>
  );
}

export { CreateGameForm };
