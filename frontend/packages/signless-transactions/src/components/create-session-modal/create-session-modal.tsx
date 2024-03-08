import { Button, Input, Modal, ModalProps, Select } from '@gear-js/vara-ui';
import { useApi, useBalanceFormat } from '@gear-js/react-hooks';
import { GearKeyring, decodeAddress } from '@gear-js/api';
import { KeyringPair, KeyringPair$Json } from '@polkadot/keyring/types';
import { ChangeEvent, useEffect, useMemo, useState } from 'react';
import { useForm } from 'react-hook-form';
import { useSignlessTransactions } from '../../context';
import { getMilliseconds } from '../../utils';
import styles from './create-session-modal.module.css';
import { SignlessParams } from '../signless-params-list';
import { AccountPair } from '../account-pair';
import {
  ACTIONS,
  BALANCE_VALUE_TO_ISSUE_VOUCHER,
  BALANCE_VALUE_TO_START_GAME,
  DEFAULT_VALUES,
  DURATIONS,
  REQUIRED_MESSAGE,
} from '@/consts';

type Props = Pick<ModalProps, 'close'>;

function CreateSessionModal({ close }: Props) {
  const { api } = useApi();
  const { getChainBalanceValue, getFormattedBalance } = useBalanceFormat();
  const [durationMinutes, setDurationMinutes] = useState<number>(DURATIONS[0].value);
  const { register, handleSubmit, formState, setError } = useForm({ defaultValues: DEFAULT_VALUES });
  const { errors } = formState;

  const {
    savePair,
    storagePair,
    voucherBalance,
    createSession,
    updateSession,
    pair: existingPair,
  } = useSignlessTransactions();
  // eslint-disable-next-line react-hooks/exhaustive-deps
  const [pair, setPair] = useState<KeyringPair | KeyringPair$Json | undefined>(storagePair);

  useEffect(() => {
    if (pair) return;

    GearKeyring.create('signlessPair').then((result) => setPair(result.keyring));
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const [isLoading, setIsLoading] = useState(false);

  const issueVoucherValue = useMemo(() => {
    if (!api) throw new Error('API is not initialized');

    const minValue = api.existentialDeposit.toNumber();

    const valueToStart = getChainBalanceValue(BALANCE_VALUE_TO_START_GAME).toNumber();
    const valueToIssueVoucher = getChainBalanceValue(BALANCE_VALUE_TO_ISSUE_VOUCHER).toNumber();

    const totalValueToStart = minValue + valueToStart;
    const totalValueToIssueVoucher = minValue + valueToIssueVoucher;

    return voucherBalance < totalValueToIssueVoucher ? totalValueToStart - voucherBalance : 0;
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [api, voucherBalance]);

  const formattedIssueVoucherValue = getFormattedBalance(issueVoucherValue);

  const onSubmit = (values: typeof DEFAULT_VALUES) => {
    if (!pair) return;

    setIsLoading(true);

    const { password } = values;
    const duration = getMilliseconds(durationMinutes);
    const key = decodeAddress(pair.address);
    const allowedActions = ACTIONS;

    const onSuccess = async () => {
      if (storagePair) {
        if (!existingPair) {
          try {
            const pairFromStorageJSON = GearKeyring.fromJson(storagePair, password);
            savePair(pairFromStorageJSON as KeyringPair, password);
            close();
          } catch (error) {
            const message = String(error);
            setError('password', { message });
          }
        } else {
          close();
        }
      } else {
        savePair(pair as KeyringPair, password);
        close();
      }
    };

    const onFinally = () => setIsLoading(false);

    if (storagePair) {
      updateSession({ duration, key, allowedActions }, issueVoucherValue, { onSuccess, onFinally });
      return;
    }

    createSession({ duration, key, allowedActions }, issueVoucherValue, { onSuccess, onFinally });
  };

  const handleSelectChange = (e: ChangeEvent<HTMLSelectElement>) => {
    setDurationMinutes(Number(e.target.value));
  };

  return (
    <>
      <Modal heading={`${storagePair ? 'Resume' : 'Enable'} Signless Session`} close={close}>
        <SignlessParams
          params={[
            {
              heading: storagePair ? 'Account from the storage:' : 'Randomly generated account:',
              value: pair ? <AccountPair pair={pair} /> : <span />,
            },
            {
              heading: 'Voucher to issue:',
              value: `${formattedIssueVoucherValue.value} ${formattedIssueVoucherValue.unit}`,
            },
          ]}
        />

        <form onSubmit={handleSubmit(onSubmit)} className={styles.form}>
          <Select label="Session duration" options={DURATIONS} onChange={handleSelectChange} />
          {(!storagePair || !existingPair) && (
            <Input
              type="password"
              label="Set password"
              error={errors.password?.message}
              {...register('password', {
                required: REQUIRED_MESSAGE,
                minLength: { value: 6, message: 'Minimum length is 6' },
              })}
            />
          )}
          <Button type="submit" text="Create Signless session" className={styles.button} isLoading={isLoading} />
        </form>
      </Modal>
    </>
  );
}

export { CreateSessionModal };
