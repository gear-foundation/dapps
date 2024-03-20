import { Button, Input, Modal, ModalProps, Select } from '@gear-js/vara-ui';
import { useApi, useBalanceFormat } from '@gear-js/react-hooks';
import { decodeAddress } from '@gear-js/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { useMemo, useState } from 'react';
import { useForm } from 'react-hook-form';
import { useSignlessTransactions } from '../../context';
import { getMilliseconds, getRandomPair, getUnlockedPair } from '../../utils';
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
} from '../../consts';

type Props = Pick<ModalProps, 'close'> & {
  onSessionCreate?: (signlessAccountAddress: string) => Promise<void>;
  shouldIssueVoucher?: boolean;
};

function CreateSessionModal({ close, onSessionCreate = async () => {}, shouldIssueVoucher = true }: Props) {
  const { api } = useApi();
  const { getChainBalanceValue, getFormattedBalance } = useBalanceFormat();

  const { register, handleSubmit, formState, setError } = useForm({ defaultValues: DEFAULT_VALUES });
  const { errors } = formState;

  const { savePair, storagePair, voucherBalance, createSession, updateSession } = useSignlessTransactions();
  const pair = useMemo(() => storagePair || getRandomPair(), [storagePair]);

  const [isLoading, setIsLoading] = useState(false);

  const issueVoucherValue = useMemo(() => {
    if (!api) throw new Error('API is not initialized');
    if (!shouldIssueVoucher) return 0;

    const minValue = api.existentialDeposit.toNumber();

    const valueToStart = getChainBalanceValue(BALANCE_VALUE_TO_START_GAME).toNumber();
    const valueToIssueVoucher = getChainBalanceValue(BALANCE_VALUE_TO_ISSUE_VOUCHER).toNumber();

    const totalValueToStart = minValue + valueToStart;
    const totalValueToIssueVoucher = minValue + valueToIssueVoucher;

    return voucherBalance < totalValueToIssueVoucher ? totalValueToStart - voucherBalance : 0;
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [api, voucherBalance]);

  const formattedIssueVoucherValue = getFormattedBalance(issueVoucherValue);

  const onSubmit = async ({ password, durationMinutes }: typeof DEFAULT_VALUES) => {
    const duration = getMilliseconds(Number(durationMinutes));
    const key = decodeAddress(pair.address);
    const allowedActions = ACTIONS;

    const callSession = storagePair ? updateSession : createSession;
    const onFinally = () => setIsLoading(false);

    let pairToSave: KeyringPair;

    setIsLoading(true);

    try {
      pairToSave = storagePair ? getUnlockedPair(storagePair, password) : (pair as KeyringPair);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);

      setError('password', { message });
      return onFinally();
    }

    const onSuccess = () => {
      savePair(pairToSave, password);
      close();
    };

    if (!shouldIssueVoucher) await onSessionCreate(pairToSave.address);

    callSession({ duration, key, allowedActions }, issueVoucherValue, { onSuccess, onFinally });
  };

  return (
    <>
      <Modal heading={storagePair ? 'Resume Signless Session' : 'Create Signless Session'} close={close}>
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
          <Select label="Session duration" options={DURATIONS} {...register('durationMinutes')} />

          <Input
            type="password"
            label="Set password"
            error={errors.password?.message}
            {...register('password', {
              required: REQUIRED_MESSAGE,
              minLength: { value: 6, message: 'Minimum length is 6' },
            })}
          />

          <Button type="submit" text="Submit" className={styles.button} isLoading={isLoading} />
        </form>
      </Modal>
    </>
  );
}

export { CreateSessionModal };
