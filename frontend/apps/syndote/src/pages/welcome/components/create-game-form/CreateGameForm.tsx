import { Button } from '@gear-js/vara-ui';
import { decodeAddress } from '@gear-js/api';
import { ReactComponent as VaraSVG } from 'assets/images/icons/vara-coin.svg';
import { ReactComponent as TVaraSVG } from 'assets/images/icons/tvara-coin.svg';
import { useAccount, useApi, useBalanceFormat } from '@gear-js/react-hooks';
import { TextField } from 'components/layout/text-field';
import { isNotEmpty, useForm } from '@mantine/form';
import { useCreateGameSessionMessage } from 'app/utils';
import { usePending } from 'app/hooks';
import styles from './CreateGameForm.module.scss';

type CreateFormValues = {
  fee: number;
  name: string;
  strategyId: string;
};

type Props = {
  onCancel: () => void;
};

function CreateGameForm({ onCancel }: Props) {
  const { account } = useAccount();
  const { api } = useApi();
  const { getFormattedBalanceValue, getChainBalanceValue } = useBalanceFormat();

  const { createGameSessionMessage } = useCreateGameSessionMessage();

  const { pending } = usePending();
  const existentialDeposit = Number(getFormattedBalanceValue(api?.existentialDeposit.toNumber() || 0).toFixed());

  const createForm = useForm({
    initialValues: {
      fee: existentialDeposit + 5 || 0,
      name: '',
      strategyId: '',
    },
    validate: {
      fee: (value) =>
        Number(value) < existentialDeposit + 5 && Number(value) > 0
          ? `value must be more than ${existentialDeposit + 5} or 0`
          : null,
      name: isNotEmpty(`Name shouldn't be empty`),
      strategyId: (val) => !val.trim().startsWith('0x') && 'Incorrect program address',
    },
  });

  const { errors: createErrors, getInputProps: getCreateInputProps, onSubmit: onCreateSubmit } = createForm;

  const handleCreateSession = (values: CreateFormValues) => {
    if (!account?.decodedAddress) {
      return;
    }
    createGameSessionMessage({
      value: Number(values.fee) ? BigInt(getChainBalanceValue(values.fee).toFixed()) : undefined,
      name: values.name,
      strategyId: decodeAddress(values.strategyId),
      entryFee: Number(values.fee) ? values.fee * Math.pow(10, 12) : null,
    });
  };

  return (
    <form className={styles.form} onSubmit={onCreateSubmit(handleCreateSession)}>
      <div className={styles.input}>
        <TextField
          theme="dark"
          label="Enter your program address:"
          placeholder="0x25c..."
          variant="active"
          disabled={pending}
          {...getCreateInputProps('strategyId')}
        />
        <span className={styles.fieldError}>{createErrors.strategyId}</span>
      </div>
      <div className={styles.input}>
        <TextField
          label="Entry fee"
          variant="active"
          type="number"
          icon={api?.registry.chainTokens[0].toLowerCase() === 'vara' ? <VaraSVG /> : <TVaraSVG />}
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
      <div className={styles.buttons}>
        <Button type="submit" text="Continue" disabled={pending} className={styles.button} />
        <Button
          type="submit"
          text="Cancel"
          color="dark"
          disabled={pending}
          className={styles.button}
          onClick={onCancel}
        />
      </div>
    </form>
  );
}

export { CreateGameForm };
