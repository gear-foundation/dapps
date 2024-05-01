import { useState } from 'react';
import { Modal } from 'components/layout/modal';
import { ReactComponent as VaraSVG } from 'assets/images/icons/vara-coin.svg';
import { ReactComponent as TVaraSVG } from 'assets/images/icons/tvara-coin.svg';
import { useAccountDeriveBalancesAll, useApi, useBalanceFormat } from '@gear-js/react-hooks';
import { TextField } from 'components/layout/text-field';
import { Button } from '@gear-js/vara-ui';
import { isNotEmpty, useForm } from '@mantine/form';
import { ReactComponent as UserSVG } from 'assets/images/icons/ic-user-small-24.svg';
import styles from './GameFoundModal.module.scss';
import { GameDetails } from '../../../components/layout/game-details/GameDetails';

type Props = {
  entryFee: number | string;
  players: number;
  gasAmount: number | string;
  onSubmit: (values: JoinModalFormValues) => void;
  onClose: () => void;
};

export type JoinModalFormValues = {
  name: string;
  strategyId: string;
};

function GameFoundModal({ entryFee, players, gasAmount, onSubmit, onClose }: Props) {
  const { isApiReady } = useApi();
  const [isLoading, setIsLoading] = useState<boolean>(false);
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
          <UserSVG /> {players} / 4
        </>
      ),
      key: '2',
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
      strategyId: '',
    },
    validate: {
      name: isNotEmpty(`Name shouldn't be empty`),
      strategyId: (val) => !/^0x|^kG/.test(val.trim()) && 'Incorrect program address',
    },
  });

  const { errors: joinErrors, getInputProps: getJoinInputProps, onSubmit: onJoinSubmit } = joinForm;

  const handleJoinSession = (values: JoinModalFormValues) => {
    onSubmit(values);
  };

  return (
    <Modal heading="The game has been found" onClose={onClose}>
      <div className={styles.container}>
        <p>
          To proceed, review the parameters of the gaming session and click the “Join” button. If applicable, you will
          need to pay the entry fee and required amount of gas immediately after clicking the “Join” button. After the
          end of the game, any unused gas will be refunded.
        </p>
        <GameDetails items={items} />
        <form className={styles.form} onSubmit={onJoinSubmit(handleJoinSession)}>
          <div className={styles.input}>
            <TextField
              theme="dark"
              label="Enter your program address:"
              placeholder="0x25c..."
              variant="active"
              {...getJoinInputProps('strategyId')}
            />
            <span className={styles.fieldError}>{joinErrors.strategyId}</span>
          </div>
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
            <Button text="Cancel" color="dark" disabled={isLoading} className={styles.button} onClick={onClose} />
            <Button type="submit" text="Join" disabled={isLoading} className={styles.button} />
          </div>
        </form>
      </div>
    </Modal>
  );
}

export { GameFoundModal };
