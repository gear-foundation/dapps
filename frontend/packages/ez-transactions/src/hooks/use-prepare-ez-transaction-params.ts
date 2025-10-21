import { HexString } from '@gear-js/api';
import { useAccount, useApi, useBalanceFormat } from '@gear-js/react-hooks';
import { IKeyringPair } from '@polkadot/types/types';
import { useCallback } from 'react';

import { useEzTransactions } from '../context';
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
};

const usePrepareEzTransactionParams = () => {
  const { account } = useAccount();
  const { signless, gasless } = useEzTransactions();
  const { voucherReissueThreshold, storageVoucherBalance, isSessionActive } = signless;
  const isAutoSignlessEnabledGlobal = signless.isAutoSignlessEnabled;
  const { signlessSnapshotRef, gaslessSnapshotRef } = useContextSnapshots(signless, gasless);
  const { handleAutoSignless } = useAutoSignless(signless);
  const { getChainBalanceValue } = useBalanceFormat();
  const { api } = useApi();

  const prepareEzTransactionParams = useCallback(
    async (prepareOptions?: PrepareEzTransactionParamsOptions): Promise<PrepareEzTransactionParamsResult> => {
      if (!account) throw new Error('Account not found');
      if (!api) throw new Error('API not found');

      const { sendFromBaseAccount, autoSignless: autoSignlessOverrides } = prepareOptions ?? {};
      const gaslessState = gaslessSnapshotRef.current;

      const minValue = api.existentialDeposit.toNumber();
      const _valueToIssueVoucher = getChainBalanceValue(voucherReissueThreshold).toNumber();
      const valueToIssueVoucher = Math.max(minValue, _valueToIssueVoucher);
      const shouldUpdateVoucherBalance = isSessionActive && storageVoucherBalance < valueToIssueVoucher;

      const shouldHandleAutoSignless = prepareOptions?.isAutoSignlessEnabled ?? isAutoSignlessEnabledGlobal;

      const options = {
        ...autoSignlessOverrides,
        allowedActions: autoSignlessOverrides?.allowedActions ?? signlessSnapshotRef.current.allowedActions ?? [],
        shouldIssueVoucher: !gaslessState.isEnabled,
        onSessionCreate: (signlessAccountAddress: string) => gaslessState.requestVoucher(signlessAccountAddress),
      };

      if (shouldUpdateVoucherBalance) {
        if (gaslessState.isEnabled) {
          // TODO: realize this logic for gasless + signless mode
          // TODO: delete old session and create new one with updated voucher
          console.log('Gassless voucher has low balance, you should update signless session manually');
        } else {
          await signlessSnapshotRef.current.openSessionModal({ ...options, type: 'topup-balance' });
        }
      }

      if (shouldHandleAutoSignless && !shouldUpdateVoucherBalance) {
        await handleAutoSignless(options);
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
    [
      account,
      isAutoSignlessEnabledGlobal,
      handleAutoSignless,
      signlessSnapshotRef,
      gaslessSnapshotRef,
      voucherReissueThreshold,
      getChainBalanceValue,
      api,
      storageVoucherBalance,
      isSessionActive,
    ],
  );

  return { prepareEzTransactionParams };
};

export { usePrepareEzTransactionParams, type PrepareEzTransactionParamsResult, type PrepareEzTransactionParamsOptions };
