import { HexString } from '@gear-js/api';
import { useAccount } from '@gear-js/react-hooks';
import { IKeyringPair } from '@polkadot/types/types';
import { useCallback } from 'react';

import { useEzTransactions } from '../context';
import type { GetPendingTransaction } from '../features/signless-transactions/context/types';
import { useAutoSignless, type AutoSignlessOptions } from '../features/signless-transactions/hooks/use-auto-signless';

import { useContextSnapshots } from './use-context-snapshots';

type PrepareEzTransactionParamsResult = {
  sessionForAccount: HexString | null;
  account: { addressOrPair: IKeyringPair } | undefined;
  voucherId: HexString | undefined;
  gasLimit: { increaseGas: number };
};

type PrepareEzTransactionParamsOptions = {
  sendFromBaseAccount?: boolean;
  isAutoSignlessEnabled?: boolean;
  autoSignless?: AutoSignlessOptions;
  getPendingTransaction?: GetPendingTransaction;
};

const usePrepareEzTransactionParams = () => {
  const { account } = useAccount();
  const { signless, gasless } = useEzTransactions();
  const isAutoSignlessEnabledGlobal = signless.isAutoSignlessEnabled;
  const { signlessSnapshotRef, gaslessSnapshotRef } = useContextSnapshots(signless, gasless);
  const { handleAutoSignless } = useAutoSignless(signless);

  const prepareEzTransactionParams = useCallback(
    async (prepareOptions?: PrepareEzTransactionParamsOptions): Promise<PrepareEzTransactionParamsResult> => {
      if (!account) throw new Error('Account not found');

      const { sendFromBaseAccount, autoSignless: autoSignlessOverrides, getPendingTransaction } = prepareOptions ?? {};
      const gaslessState = gaslessSnapshotRef.current;

      const shouldHandleAutoSignless = prepareOptions?.isAutoSignlessEnabled ?? isAutoSignlessEnabledGlobal;

      if (shouldHandleAutoSignless) {
        const autoSignlessWithPendingTransaction = {
          ...autoSignlessOverrides,
          allowedActions: autoSignlessOverrides?.allowedActions ?? signlessSnapshotRef.current.allowedActions,
          getPendingTransaction,
          shouldIssueVoucher: !gaslessState.isEnabled,
          onSessionCreate: (signlessAccountAddress: string) => gaslessState.requestVoucher(signlessAccountAddress),
        };
        await handleAutoSignless(autoSignlessWithPendingTransaction);
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
        !gaslessSnapshotRef.current.voucherId &&
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
  type GetPendingTransaction,
};
