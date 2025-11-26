import { decodeAddress } from '@gear-js/api';
import { useApi, useBalanceFormat, useAccount, useAlert } from '@gear-js/react-hooks';
import { Button, Input, Modal, Select } from '@gear-js/vara-ui';
import { KeyringPair } from '@polkadot/keyring/types';
import { useMemo, useState } from 'react';
import { useForm } from 'react-hook-form';

import { DURATIONS, REQUIRED_MESSAGE } from '../../consts';
import { useSignlessTransactions } from '../../context';
import { useRandomPairOr } from '../../hooks';
import { getMilliseconds, getMinutesFromSeconds, getUnlockedPair } from '../../utils';
import { AccountPair } from '../account-pair';
import { SignlessParams } from '../signless-params-list';

import styles from './create-session-modal.module.css';

type Props = {
  allowedActions: string[];
  onSessionCreate?: (signlessAccountAddress: string) => Promise<`0x${string}`>;
  shouldIssueVoucher?: boolean; // no need to pass boolean, we can just conditionally pass onSessionCreate?
  boundSessionDuration?: number;
  defaultDurationMinutes?: string;
  maxWidth?: 'small' | 'medium' | 'large' | (string & NonNullable<unknown>);
  modalType?: 'create' | 'topup-balance';
  close: (success?: boolean) => void;
};

function CreateSessionModal({
  allowedActions,
  close,
  onSessionCreate = () => Promise.resolve('0x'),
  shouldIssueVoucher = true,
  boundSessionDuration,
  defaultDurationMinutes,
  maxWidth,
  modalType = 'create',
}: Props) {
  const { api } = useApi();
  const { account } = useAccount();
  const alert = useAlert();
  const { getChainBalanceValue, getFormattedBalance } = useBalanceFormat();

  const gaslessVoucherDurationMinutes = boundSessionDuration ? getMinutesFromSeconds(boundSessionDuration) : undefined;

  const DURATION_OPTIONS = useMemo(
    () =>
      boundSessionDuration
        ? [
            {
              label: `${gaslessVoucherDurationMinutes} minutes`,
              value: gaslessVoucherDurationMinutes,
            },
          ]
        : DURATIONS,
    [gaslessVoucherDurationMinutes, boundSessionDuration],
  );

  const {
    savePair,
    storagePair,
    storageVoucher,
    storageVoucherBalance,
    createSession,
    updateVoucherBalance,
    session,
    voucherIssueAmount,
    voucherReissueThreshold,
    allowIncreaseVoucherValue,
  } = useSignlessTransactions();

  const DEFAULT_VALUES = useMemo(() => {
    let durationValue = DURATIONS[0].value;

    if (gaslessVoucherDurationMinutes) {
      durationValue = `${gaslessVoucherDurationMinutes}`;
    } else if (defaultDurationMinutes) {
      // Check if the provided default duration exists in DURATIONS
      const durationExists = DURATIONS.some((duration) => duration.value === defaultDurationMinutes);
      if (durationExists) {
        durationValue = defaultDurationMinutes;
      }
    }

    return {
      password: '',
      durationMinutes: durationValue,
      voucherAmount: voucherIssueAmount,
    };
  }, [gaslessVoucherDurationMinutes, defaultDurationMinutes, voucherIssueAmount]);

  const { register, handleSubmit, formState, setError, watch } = useForm({ defaultValues: DEFAULT_VALUES });
  const { errors } = formState;
  const customVoucherAmount = watch('voucherAmount');

  const pair = useRandomPairOr(storagePair);

  const [isLoading, setIsLoading] = useState(false);

  const issueVoucherValue = useMemo(() => {
    if (!account) throw new Error('Account is not initialized');
    if (!api) throw new Error('API is not initialized');
    if (!shouldIssueVoucher) return 0;

    const minValue = api.existentialDeposit.toNumber();

    const amountToUse = allowIncreaseVoucherValue ? customVoucherAmount || 0 : voucherIssueAmount;
    const _valueToStart = getChainBalanceValue(amountToUse).toNumber();
    const valueToStart = Math.max(minValue, _valueToStart);
    const _valueToIssueVoucher = getChainBalanceValue(voucherReissueThreshold).toNumber();
    const valueToIssueVoucher = Math.max(minValue, _valueToIssueVoucher);

    const isOwner = storageVoucher?.owner === account.decodedAddress;
    if (!isOwner) return valueToStart;

    return storageVoucherBalance < valueToIssueVoucher ? valueToStart - storageVoucherBalance : 0;
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [api, storageVoucherBalance, shouldIssueVoucher, customVoucherAmount, allowIncreaseVoucherValue]);

  const formattedIssueVoucherValue = getFormattedBalance(issueVoucherValue);

  const onSubmit = async ({ password, durationMinutes }: typeof DEFAULT_VALUES) => {
    if (!pair) throw new Error('Signless pair is not initialized');
    if (!account) throw new Error('Account not found');

    const duration = getMilliseconds(Number(durationMinutes));

    const key = shouldIssueVoucher ? decodeAddress(pair.address) : account.decodedAddress;
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
      close(true);
    };

    console.log('SUBMITTING');
    console.log('modalType: ', modalType);
    console.log('shouldIssueVoucher: ', shouldIssueVoucher);
    console.log('issueVoucherValue', issueVoucherValue);

    // Update voucher balance mode - only update balance without creating new session
    if (modalType === 'topup-balance') {
      if (!session) throw new Error('Session not found for balance update');

      try {
        await updateVoucherBalance(
          { duration: 0, key: session.key, allowedActions: session.allowedActions },
          issueVoucherValue,
          { onSuccess: () => close(true), onFinally },
        );
      } catch (error) {
        const message = error instanceof Error ? error.message : 'Failed to update voucher balance';
        alert.error(message);
        onFinally();
      }

      return;
    }

    if (!shouldIssueVoucher) {
      try {
        const voucherId = await onSessionCreate(pairToSave.address);

        console.log('voucherId', voucherId);
        console.log('pairToSave: ', pairToSave);

        await createSession({ duration, key, allowedActions }, issueVoucherValue, {
          shouldIssueVoucher,
          voucherId,
          onSuccess,
          onFinally,
          pair: pairToSave,
        });
      } catch (_err) {
        alert.error('Error when fetching gasless voucher');
        onFinally();
      }

      return;
    }

    await createSession({ duration, key, allowedActions }, issueVoucherValue, {
      shouldIssueVoucher,
      onSuccess,
      onFinally,
    });
  };

  const getModalHeading = () => {
    if (modalType === 'topup-balance') return 'Top Up Voucher Balance';
    if (storagePair) return 'Resume Signless Session';
    return 'Create Signless Session';
  };

  return (
    <>
      <Modal heading={getModalHeading()} close={close} maxWidth={maxWidth}>
        <SignlessParams
          params={[
            {
              heading: storagePair ? 'Account from the storage:' : 'Randomly generated account:',
              value: pair ? <AccountPair pair={pair} /> : <span />,
            },
            {
              heading: 'Voucher to issue:',
              value:
                issueVoucherValue > 0 ? `${formattedIssueVoucherValue.value} ${formattedIssueVoucherValue.unit}` : '',
            },
          ]}
        />

        <form onSubmit={handleSubmit(onSubmit)} className={styles.form}>
          {modalType !== 'topup-balance' && (
            <Select
              label="Session duration"
              disabled={!!boundSessionDuration}
              options={DURATION_OPTIONS}
              {...register('durationMinutes')}
            />
          )}

          {allowIncreaseVoucherValue &&
            shouldIssueVoucher &&
            (issueVoucherValue > 0 || modalType === 'topup-balance') && (
              <Input
                type="number"
                label="Voucher Amount to Issue"
                error={errors.voucherAmount?.message}
                {...register('voucherAmount', {
                  valueAsNumber: true,
                  required: REQUIRED_MESSAGE,
                  min: { value: voucherIssueAmount, message: `Minimum value is ${voucherIssueAmount}` },
                })}
              />
            )}

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
