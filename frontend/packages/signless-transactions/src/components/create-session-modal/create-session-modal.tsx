import { Button, Input, Modal, ModalProps, Select } from '@gear-js/vara-ui';
import { useApi, useBalanceFormat, useAccount } from '@gear-js/react-hooks';
import { decodeAddress } from '@gear-js/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { useMemo, useState } from 'react';
import { useForm } from 'react-hook-form';
import { useRandomPairOr } from '@/hooks';
import { useSignlessTransactions } from '../../context';
import { getMilliseconds, getUnlockedPair } from '../../utils';
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
  shouldIssueVoucher?: boolean; // no need to pass boolean, we can just conditionally pass onSessionCreate?
};

function CreateSessionModal({ close, onSessionCreate = async () => {}, shouldIssueVoucher = true }: Props) {
  const { api } = useApi();
  const { account } = useAccount();
  const { getChainBalanceValue, getFormattedBalance } = useBalanceFormat();

  const { register, handleSubmit, formState, setError } = useForm({ defaultValues: DEFAULT_VALUES });
  const { errors } = formState;

  const { savePair, storagePair, storageVoucher, storageVoucherBalance, createSession } = useSignlessTransactions();
  const pair = useRandomPairOr(storagePair);

  const [isLoading, setIsLoading] = useState(false);

  const issueVoucherValue = useMemo(() => {
    if (!account) throw new Error('Account is not initialized');
    if (!api) throw new Error('API is not initialized');
    if (!shouldIssueVoucher) return 0;

    const minValue = api.existentialDeposit.toNumber();

    const valueToStart = getChainBalanceValue(BALANCE_VALUE_TO_START_GAME).toNumber();
    const valueToIssueVoucher = getChainBalanceValue(BALANCE_VALUE_TO_ISSUE_VOUCHER).toNumber();

    const totalValueToStart = minValue + valueToStart;

    const isOwner = storageVoucher?.owner === account.decodedAddress;
    if (!isOwner) return totalValueToStart;

    const totalValueToIssueVoucher = minValue + valueToIssueVoucher;

    return storageVoucherBalance < totalValueToIssueVoucher ? totalValueToStart - storageVoucherBalance : 0;
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [api, storageVoucherBalance, shouldIssueVoucher]);

  const formattedIssueVoucherValue = getFormattedBalance(issueVoucherValue);

  const onSubmit = async ({ password, durationMinutes }: typeof DEFAULT_VALUES) => {
    if (!pair) throw new Error('Signless pair is not initialized');

    const duration = getMilliseconds(Number(durationMinutes));
    const key = decodeAddress(pair.address);
    const allowedActions = ACTIONS;
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

    createSession({ duration, key, allowedActions }, issueVoucherValue, { shouldIssueVoucher, onSuccess, onFinally });
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
