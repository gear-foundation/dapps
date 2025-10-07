import { HexString } from '@gear-js/api';
import { useAccount } from '@gear-js/react-hooks';
import { IKeyringPair } from '@polkadot/types/types';
import { useCallback, useEffect, useRef } from 'react';

import { useEzTransactions } from '../context';
import { useAutoSignless, type AutoSignlessOptions } from '../features/signless-transactions/hooks/use-auto-signless';

import { useContextSnapshots } from './use-context-snapshots';

type PrepareEzTransactionParamsResult = {
  sessionForAccount: HexString | null;
  account: { addressOrPair: IKeyringPair } | undefined;
  voucherId: HexString | undefined;
  gasLimit: { increaseGas: number };
};

type UsePrepareEzTransactionParamsOptions = {
  isAutoSignlessEnabled?: boolean;
  autoSignless?: AutoSignlessOptions;
};

type PrepareEzTransactionParamsOptions = {
  sendFromBaseAccount?: boolean;
  isAutoSignlessEnabled?: boolean;
  autoSignless?: AutoSignlessOptions;
};

const usePrepareEzTransactionParams = (options?: UsePrepareEzTransactionParamsOptions) => {
  const { account } = useAccount();
  const { signless, gasless, isAutoSignlessEnabled: isAutoSignlessEnabledGlobal } = useEzTransactions();

  const defaultIsAutoSignlessEnabledRef = useRef<boolean | undefined>(options?.isAutoSignlessEnabled);
  const { signlessSnapshotRef, gaslessSnapshotRef } = useContextSnapshots(signless, gasless);
  const { handleAutoSignless } = useAutoSignless(signless, options?.autoSignless);

  useEffect(() => {
    defaultIsAutoSignlessEnabledRef.current = options?.isAutoSignlessEnabled;
  }, [options?.isAutoSignlessEnabled]);

  const prepareEzTransactionParams = useCallback(
    async (prepareOptions?: PrepareEzTransactionParamsOptions): Promise<PrepareEzTransactionParamsResult> => {
      if (!account) throw new Error('Account not found');

      const { sendFromBaseAccount, autoSignless: autoSignlessOverrides } = prepareOptions ?? {};
      const gaslessState = gaslessSnapshotRef.current;

      const shouldHandleAutoSignless =
        prepareOptions?.isAutoSignlessEnabled ?? defaultIsAutoSignlessEnabledRef.current ?? isAutoSignlessEnabledGlobal;

      if (shouldHandleAutoSignless) {
        await handleAutoSignless(autoSignlessOverrides);
      }

      const nextSignlessState = signlessSnapshotRef.current;
      const pair = nextSignlessState.pair;
      const voucher = nextSignlessState.voucher;
      const sendFromPair = Boolean(pair && voucher?.id && !sendFromBaseAccount);
      const sessionForAccount = sendFromPair ? account.decodedAddress : null;

      let voucherId = sendFromPair ? voucher?.id : gaslessState.voucherId;
      if (
        account &&
        gaslessState.isEnabled &&
        !gaslessState.voucherId &&
        (sendFromBaseAccount || !nextSignlessState.isActive)
      ) {
        voucherId = await gaslessState.requestVoucher(account.address);
      }

      return {
        sessionForAccount,
        account: sendFromPair && pair ? { addressOrPair: pair } : undefined,
        voucherId,
        gasLimit: { increaseGas: 10 },
      };
    },
    [account, isAutoSignlessEnabledGlobal, handleAutoSignless, signlessSnapshotRef, gaslessSnapshotRef],
  );

  return { prepareEzTransactionParams };
};

export {
  usePrepareEzTransactionParams,
  type PrepareEzTransactionParamsResult,
  type PrepareEzTransactionParamsOptions,
  type UsePrepareEzTransactionParamsOptions,
};
