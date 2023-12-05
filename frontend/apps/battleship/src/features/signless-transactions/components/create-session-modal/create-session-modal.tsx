import { Button, Input, Modal, ModalProps } from '@gear-js/vara-ui';
import { useApi, useBalanceFormat } from '@gear-js/react-hooks';
import { decodeAddress } from '@gear-js/api';
import { KeyringPair } from '@polkadot/keyring/types';
import Identicon from '@polkadot/react-identicon';
import { useMemo, useState } from 'react';
import { useForm } from 'react-hook-form';

import { ADDRESS } from '@/app/consts';

import { useSignlessTransactions } from '../../context';
import { useCreateSession, useIssueVoucher } from '../../hooks';
import { getMilliseconds, getRandomPair } from '../../utils';
import styles from './create-session-modal.module.css';

type Props = Pick<ModalProps, 'close'>;

const DEFAULT_VALUES = { password: '' };
const REQUIRED_MESSAGE = 'Field is required';

const DURATION_MINUTES = 5;
const VOUCHER_VALUE = 10;
const ACTIONS = ['StartGame', 'Turn'];

function CreateSessionModal({ close }: Props) {
  const { api } = useApi();
  const [unit] = api?.registry.chainTokens || ['Unit'];
  const { getChainBalanceValue } = useBalanceFormat();

  const { register, handleSubmit, formState } = useForm({ defaultValues: DEFAULT_VALUES });
  const { errors } = formState;

  const { savePair, storagePair } = useSignlessTransactions();
  // eslint-disable-next-line react-hooks/exhaustive-deps
  const pair = useMemo(() => storagePair || getRandomPair(), []);

  const { createSession, deleteSession } = useCreateSession();
  const issueVoucher = useIssueVoucher();

  const [isLoading, setIsLoading] = useState(false);
  const disableLoading = () => setIsLoading(false);

  const onSubmit = (values: typeof DEFAULT_VALUES) => {
    setIsLoading(true);

    const { password } = values;
    const value = getChainBalanceValue(VOUCHER_VALUE).toFixed();
    const duration = getMilliseconds(DURATION_MINUTES);
    const decodedAddress = decodeAddress(pair.address);

    const onVoucherSuccess = () => {
      if (!storagePair) savePair(pair as KeyringPair, password);

      close();
    };

    const onVoucherError = () => {
      deleteSession();
      disableLoading();
    };

    const onCreateSuccess = () => issueVoucher(ADDRESS.GAME, decodedAddress, value, onVoucherSuccess, onVoucherError);

    createSession(decodedAddress, duration, ACTIONS, onCreateSuccess, disableLoading);
  };

  return (
    <Modal heading="Create Signless Session" close={close}>
      <ul className={styles.summary}>
        <li>
          <h4 className={styles.heading}>
            {storagePair ? 'Account from the storage:' : 'Randomly generated account:'}
          </h4>

          <div className={styles.account}>
            <Identicon value={pair.address} theme="polkadot" size={14} />
            <span>{pair.address}</span>
          </div>
        </li>

        <li>
          <h4 className={styles.heading}>Voucher to issue:</h4>
          <p>
            {VOUCHER_VALUE} {unit}
          </p>
        </li>

        <li>
          <h4 className={styles.heading}>Session duration:</h4>
          <p>{DURATION_MINUTES} minutes</p>
        </li>
      </ul>

      <form onSubmit={handleSubmit(onSubmit)} className={styles.form}>
        {!storagePair && (
          <Input
            type="password"
            label="Password"
            error={errors.password?.message}
            {...register('password', {
              required: REQUIRED_MESSAGE,
              minLength: { value: 6, message: 'Minimum length is 6' },
            })}
          />
        )}

        <Button type="submit" text="Submit" className={styles.button} isLoading={isLoading} />
      </form>
    </Modal>
  );
}

export { CreateSessionModal };
