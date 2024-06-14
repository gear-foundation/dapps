import { Button } from '@gear-js/vara-ui';
import { decodeAddress } from '@gear-js/api';
import { ReactComponent as VaraSVG } from '@/assets/images/icons/vara-coin.svg';
import { ReactComponent as TVaraSVG } from '@/assets/images/icons/tvara-coin.svg';
import { useAtom } from 'jotai';
import { isLoadingAtom } from '@/features/game/store';
import { useAccount, useAccountDeriveBalancesAll, useApi, useBalanceFormat, withoutCommas } from '@gear-js/react-hooks';
import { TextField } from '@/components/layout/text-field';
import { isNotEmpty, useForm } from '@mantine/form';
// import { useSyndoteMessage } from 'hooks/metadata';
import styles from './CreateGameForm.module.scss';
import { Heading } from '@/components/ui/heading';
import { Text } from '@/components/ui/text';
import { EzTransactionsSwitch } from '@dapps-frontend/ez-transactions';
import { SIGNLESS_ALLOWED_ACTIONS } from '@/app/consts';
import { GameDetails } from '@/components/layout/game-details';

export interface ContractFormValues {
  [key: string]: string;
}

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
  const balances = useAccountDeriveBalancesAll();
  const { isApiReady } = useApi();
  const { api } = useApi();
  const { getFormattedBalance, getFormattedBalanceValue, getChainBalanceValue } = useBalanceFormat();
  const balance =
    isApiReady && balances?.freeBalance ? getFormattedBalance(balances.freeBalance.toString()) : undefined;
  // const { isMeta, sendMessage: sendNewSessionMessage } = useSyndoteMessage();
  const [isLoading, setIsLoading] = useAtom(isLoadingAtom);
  const existentialDeposit = Number(getFormattedBalanceValue(api?.existentialDeposit.toNumber() || 0).toFixed());
  const VaraSvg = balance?.unit?.toLowerCase() === 'vara' ? <VaraSVG /> : <TVaraSVG />;

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
    },
  });

  const { errors: createErrors, getInputProps: getCreateInputProps, onSubmit: onCreateSubmit } = createForm;

  const handleCreateSession = (values: CreateFormValues) => {
    if (!account?.decodedAddress) {
      return;
    }

    const payload = {
      CreateGameSession: {
        name: values.name,
        entryFee: Number(values.fee) ? values.fee * Math.pow(10, 12) : null,
      },
    };

    setIsLoading(true);
    // sendNewSessionMessage({
    //   payload,
    //   value: Number(values.fee) ? getChainBalanceValue(values.fee).toFixed() : undefined,
    //   onSuccess: () => {
    //     setIsLoading(false);
    //   },
    //   onError: () => {
    //     console.log('error');
    //     setIsLoading(false);
    //   },
    // });
  };

  const items = [
    {
      name: 'Required gas amount ',
      value: (
        <>
          {VaraSvg} {1.21} VARA
        </>
      ),
      key: '3',
    },
  ];

  return (
    <div className={styles.formWrapper}>
      <div className={styles.header}>
        <Heading className={styles.mainHeading}>Create a private game</Heading>
        <div>
          <Text className={styles.mainText}>
            Set the game parameters and click “Сreate game”. To have players join you, you need to share your address.
          </Text>
        </div>
      </div>
      <form className={styles.form} onSubmit={onCreateSubmit(handleCreateSession)}>
        <div className={styles.input}>
          <TextField
            label="Entry fee"
            variant="active"
            type="number"
            icon={balance?.unit?.toLowerCase() === 'vara' ? <VaraSVG /> : <TVaraSVG />}
            disabled={isLoading}
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
            disabled={isLoading}
            {...getCreateInputProps('name')}
          />
          <span className={styles.fieldError}>{createErrors.name}</span>
        </div>
        <EzTransactionsSwitch allowedActions={SIGNLESS_ALLOWED_ACTIONS} />
        <GameDetails items={items} />
        <div className={styles.buttons}>
          <Button type="submit" text="Create game" disabled={isLoading} className={styles.button} />
          <Button
            type="submit"
            text="Cancel"
            color="grey"
            disabled={isLoading}
            className={styles.button}
            onClick={onCancel}
          />
        </div>
      </form>
    </div>
  );
}

export { CreateGameForm };
