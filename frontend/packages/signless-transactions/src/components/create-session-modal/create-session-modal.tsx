import { Button, Input, Modal, ModalProps, Select } from '@gear-js/vara-ui';
import { useApi, useBalanceFormat } from '@gear-js/react-hooks';
import { GearKeyring, decodeAddress } from '@gear-js/api';
import { KeyringPair, KeyringPair$Json } from '@polkadot/keyring/types';
import { useEffect, useMemo, useState } from 'react';
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

  const {
    savePair,
    storagePair,
    voucherBalance,
    createSession,
    updateSession,
    pair: existingPair,
  } = useSignlessTransactions();

  const [pair, setPair] = useState<KeyringPair | KeyringPair$Json | undefined>(storagePair);

  useEffect(() => {
    if (pair) return;

    GearKeyring.create('signlessPair').then((result) => setPair(result.keyring));
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

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

  const onSubmit = async (values: typeof DEFAULT_VALUES) => {
    if (!pair) return;

    setIsLoading(true);

    const { password, durationMinutes } = values;
    const duration = getMilliseconds(Number(durationMinutes));
    const key = decodeAddress(pair.address);
    const allowedActions = ACTIONS;

    // **ORIGINAL LOGIC**

    // const onSuccess = () => {
    //   if (storagePair) {
    //     if (!existingPair) {
    //       try {
    //         const pairFromStorageJSON = GearKeyring.fromJson(storagePair, password);
    //         savePair(pairFromStorageJSON as KeyringPair, password);
    //         close();
    //       } catch (error) {
    //         const message = String(error);
    //         setError('password', { message });
    //       }
    //     } else {
    //       close();
    //     }
    //   } else {
    //     savePair(pair as KeyringPair, password);
    //     close();
    //   }
    // };

    // const onFinally = () => setIsLoading(false);
    // onSessionCreate(pair.address);

    // **SHOWCASE LOGIC**

    try {
      const _pair = storagePair ? GearKeyring.fromJson(storagePair, password) : pair;

      // temporary? solution to demonstrate the ideal forkflow, where user:
      // checks the gasless -> starts game, or
      // checks the gasless -> creates signless session -> starts game.
      // cuz of gasless voucher balance check and update, signlessAccountAddress should be accessed somehow different.
      // good part about passing it as an argument is that signless pair is set after voucher request,
      // therefore it's requested voucher is accessible directly from the signless context via on chain call.
      await onSessionCreate(_pair.address);
    } catch (error) {
      const message = String(error);
      setError('password', { message });
    }

    if (storagePair) {
      if (!existingPair) {
        try {
          const pairFromStorageJSON = GearKeyring.fromJson(storagePair, password);
          savePair(pairFromStorageJSON as KeyringPair, password);
        } catch (error) {
          const message = String(error);
          setError('password', { message });
        }
      }
    } else {
      savePair(pair as KeyringPair, password);
    }

    const onSuccess = close;
    const onFinally = () => setIsLoading(false);

    if (storagePair) {
      updateSession({ duration, key, allowedActions }, issueVoucherValue, { onSuccess, onFinally });
      return;
    }

    createSession({ duration, key, allowedActions }, issueVoucherValue, { onSuccess, onFinally });
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
          <Select label="Session duration" options={DURATIONS} {...register('durationMinutes')} />

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
