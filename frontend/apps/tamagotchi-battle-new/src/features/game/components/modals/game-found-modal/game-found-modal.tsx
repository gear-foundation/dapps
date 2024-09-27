import { useAccountDeriveBalancesAll, useApi, useBalanceFormat } from '@gear-js/react-hooks';
import { Button, Input } from '@gear-js/vara-ui';
import { Modal } from '@/components/ui/modal';
import { ReactComponent as VaraSVG } from '@/assets/images/icons/vara-coin.svg';
import { ReactComponent as TVaraSVG } from '@/assets/images/icons/tvara-coin.svg';
import { isNotEmpty, useForm } from '@mantine/form';
import { GameDetails } from '@/components/layout';
import { UserIcon } from '@/assets/images';
import styles from './game-found-modal.module.scss';
import { MAX_PLAYERS_COUNT } from '@/app/consts';

type Props = {
  entryFee: number | string;
  onSubmit: (values: JoinModalFormValues) => Promise<void>;
  onClose: () => void;
};

export type JoinModalFormValues = {
  name: string;
};

function GameFoundModal({ entryFee, onSubmit, onClose }: Props) {
  const { isApiReady } = useApi();
  // const { pending } = usePending();
  const players = 1;
  const pending = false;
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
      name: 'Players already joined the game',
      value: (
        <>
          <UserIcon /> {players} / {MAX_PLAYERS_COUNT}
        </>
      ),
      key: '2',
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
    <Modal
      title="The game has been found"
      description="To proceed, review the parameters of the gaming session and click the “Join” button. If applicable, you will
          need to pay the entry fee and required amount of gas immediately after clicking the “Join” button. After the
          end of the game, any unused gas will be refunded."
      className={styles.modal}
      onClose={onClose}>
      <GameDetails items={items} />
      <form className={styles.form} onSubmit={onJoinSubmit(handleJoinSession)}>
        <div className={styles.input}>
          <Input label="Enter your name:" placeholder="Username" maxLength={20} {...getJoinInputProps('name')} />
          <span className={styles.fieldError}>{joinErrors.name}</span>
        </div>
        <div className={styles.buttons}>
          <Button text="Cancel" color="grey" disabled={pending} className={styles.button} onClick={onClose} />
          <Button type="submit" text="Join" disabled={pending} className={styles.button} />
        </div>
      </form>
    </Modal>
  );
}

export { GameFoundModal };
