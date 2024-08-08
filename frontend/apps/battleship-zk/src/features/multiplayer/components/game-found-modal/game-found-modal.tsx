import { Modal } from '@/components/ui/modal';
import { ReactComponent as VaraSVG } from '@/assets/images/icons/vara-coin.svg';
import { ReactComponent as TVaraSVG } from '@/assets/images/icons/tvara-coin.svg';
import { useAccountDeriveBalancesAll, useApi, useBalanceFormat } from '@gear-js/react-hooks';
import { TextField } from '@/components/layout/text-field';
import { Button } from '@gear-js/vara-ui';
import { isNotEmpty, useForm } from '@mantine/form';
import { GameDetails } from '@/components/layout/game-details';
import { EzTransactionsSwitch } from '@dapps-frontend/ez-transactions';
import { SIGNLESS_ALLOWED_ACTIONS } from '@/app/consts';
import { usePending } from '@/features/game/hooks';
import styles from './GameFoundModal.module.scss';

type Props = {
  entryFee: number | string;
  players: number;
  gasAmount: number | string;
  onSubmit: (values: JoinModalFormValues) => Promise<void>;
  onClose: () => void;
};

export type JoinModalFormValues = {
  name: string;
};

function GameFoundModal({ entryFee, players, gasAmount, onSubmit, onClose }: Props) {
  const { isApiReady } = useApi();
  const { pending } = usePending();
  const { getFormattedBalance } = useBalanceFormat();
  const balances = useAccountDeriveBalancesAll();
  const balance =
    isApiReady && balances?.freeBalance ? getFormattedBalance(balances.freeBalance.toString()) : undefined;

  const VaraSvg = balance?.unit?.toLowerCase() === 'vara' ? <VaraSVG /> : <TVaraSVG />;
  const items = [
    {
      name: 'Entry fee',
      value: (
        <>
          {VaraSvg} {entryFee} VARA
        </>
      ),
      key: '1',
    },
    {
      name: 'Required gas amount ',
      value: (
        <>
          {VaraSvg} {gasAmount} VARA
        </>
      ),
      key: '3',
    },
  ];

  const joinForm = useForm({
    initialValues: {
      name: '',
    },
    validate: {
      name: isNotEmpty(`Name shouldn't be empty`),
    },
  });

  const { errors: joinErrors, getInputProps: getJoinInputProps, onSubmit: onJoinSubmit } = joinForm;

  const handleJoinSession = async (values: JoinModalFormValues) => {
    await onSubmit(values);
  };

  return (
    <Modal heading="The game has been found" className={{ wrapper: styles.modalWrapper }} onClose={onClose}>
      <div className={styles.container}>
        <p className={styles.mainText}>
          To proceed, review the parameters of the gaming session and click the “Join” button. If applicable, you will
          need to pay the entry fee and required amount of gas immediately after clicking the “Join” button. After the
          end of the game, any unused gas will be refunded.
        </p>
        <GameDetails items={items} />
        <EzTransactionsSwitch allowedActions={SIGNLESS_ALLOWED_ACTIONS} />
        <form className={styles.form} onSubmit={onJoinSubmit(handleJoinSession)}>
          <div className={styles.input}>
            <TextField
              theme="dark"
              label="Enter your name:"
              placeholder="Username"
              variant="active"
              maxLength={20}
              {...getJoinInputProps('name')}
            />
            <span className={styles.fieldError}>{joinErrors.name}</span>
          </div>
          <div className={styles.inputs}>
            <Button type="submit" text="Join" disabled={pending} className={styles.button} />
            <Button text="Cancel" color="grey" disabled={pending} className={styles.button} onClick={onClose} />
          </div>
        </form>
      </div>
    </Modal>
  );
}

export { GameFoundModal };
