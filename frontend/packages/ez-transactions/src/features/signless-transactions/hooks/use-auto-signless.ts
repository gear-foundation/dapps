import { SubmittableExtrinsic } from '@polkadot/api/types';
import { ISubmittableResult } from '@polkadot/types/types';
import { useCallback, useEffect, useRef } from 'react';

import { useSignlessTransactions } from '@ez/features/signless-transactions';
import type { SignlessContext, SignlessSessionModalConfig } from '@ez/features/signless-transactions/context';

import { usePrepareEzTransactionParams } from '../../../hooks';
import { GetPendingTransaction } from '../context/types';

type AutoSignlessOptions = {
  allowedActions?: string[];
  shouldIssueVoucher?: boolean;
  onSessionCreate?: (signlessAccountAddress: string) => Promise<`0x${string}`>;
  boundSessionDuration?: number;
};

type ResolvedAutoSignlessOptions = Required<Pick<AutoSignlessOptions, 'allowedActions'>> & {
  shouldIssueVoucher: boolean;
  onSessionCreate?: AutoSignlessOptions['onSessionCreate'];
  boundSessionDuration?: AutoSignlessOptions['boundSessionDuration'];
};

type ExecuteWithSessionModalArg = SubmittableExtrinsic<'promise', ISubmittableResult>;

type ModalType = 'create' | 'enable';

const getResolvedOptions = (
  defaults: ResolvedAutoSignlessOptions,
  overrides?: AutoSignlessOptions,
): ResolvedAutoSignlessOptions => ({
  allowedActions: overrides?.allowedActions ?? defaults.allowedActions,
  shouldIssueVoucher: overrides?.shouldIssueVoucher ?? defaults.shouldIssueVoucher,
  onSessionCreate: overrides?.onSessionCreate ?? defaults.onSessionCreate,
  boundSessionDuration: overrides?.boundSessionDuration ?? defaults.boundSessionDuration,
});

const pickModalType = (context: Pick<SignlessContext, 'pair' | 'isSessionActive'>): ModalType =>
  context.isSessionActive && !context.pair ? 'enable' : 'create';

const toModalConfig = (type: ModalType, options: ResolvedAutoSignlessOptions): SignlessSessionModalConfig => {
  if (type === 'enable') {
    return { type: 'enable' };
  }

  return {
    type: 'create',
    allowedActions: options.allowedActions,
    shouldIssueVoucher: options.shouldIssueVoucher,
    onSessionCreate: options.onSessionCreate,
    boundSessionDuration: options.boundSessionDuration,
  };
};

const useAutoSignless = (defaultOptions?: AutoSignlessOptions) => {
  const { pair, isSessionActive, openSessionModal, isSessionReady } = useSignlessTransactions();
  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();

  const defaultsRef = useRef<ResolvedAutoSignlessOptions>({
    allowedActions: defaultOptions?.allowedActions ?? [],
    shouldIssueVoucher: defaultOptions?.shouldIssueVoucher ?? true,
    onSessionCreate: defaultOptions?.onSessionCreate,
    boundSessionDuration: defaultOptions?.boundSessionDuration,
  });

  const pairRef = useRef(pair);
  const isSessionActiveRef = useRef(isSessionActive);

  useEffect(() => {
    defaultsRef.current = {
      allowedActions: defaultOptions?.allowedActions ?? [],
      shouldIssueVoucher: defaultOptions?.shouldIssueVoucher ?? true,
      onSessionCreate: defaultOptions?.onSessionCreate,
      boundSessionDuration: defaultOptions?.boundSessionDuration,
    };
  }, [
    defaultOptions?.allowedActions,
    defaultOptions?.shouldIssueVoucher,
    defaultOptions?.onSessionCreate,
    defaultOptions?.boundSessionDuration,
  ]);

  useEffect(() => {
    pairRef.current = pair;
  }, [pair]);

  useEffect(() => {
    isSessionActiveRef.current = isSessionActive;
  }, [isSessionActive]);

  const executeWithSessionModal = useCallback(
    async (getPendingTransaction: GetPendingTransaction, options?: AutoSignlessOptions) => {
      if (!isSessionReady) return console.error('Session is not ready');

      const params = await prepareEzTransactionParams();

      const { transaction } = await getPendingTransaction(params);
      if (params.sessionForAccount) {
        return await transaction.signAndSend();
      }

      const resolvedOptions = getResolvedOptions(defaultsRef.current, options);
      const modalType = pickModalType({ pair: pairRef.current, isSessionActive: isSessionActiveRef.current });

      if (modalType === 'create' && resolvedOptions.allowedActions.length === 0) {
        throw new Error('Auto signless requires allowedActions to create a session');
      }

      const modalConfig = toModalConfig(modalType, resolvedOptions);

      await openSessionModal({ ...modalConfig, getPendingTransaction });
    },
    [isSessionReady, openSessionModal, prepareEzTransactionParams],
  );

  return { executeWithSessionModal };
};

export { useAutoSignless };
export type { AutoSignlessOptions, ExecuteWithSessionModalArg };
