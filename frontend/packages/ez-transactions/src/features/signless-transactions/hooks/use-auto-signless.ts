import { useCallback } from 'react';

import type { SignlessContext, SignlessSessionModalConfig, GetPendingTransaction } from '../context/types';

export type AutoSignlessOptions = {
  allowedActions?: string[];
  shouldIssueVoucher?: boolean;
  onSessionCreate?: (signlessAccountAddress: string) => Promise<`0x${string}`>;
  boundSessionDuration?: number;
  getPendingTransaction?: GetPendingTransaction;
};

type ModalType = 'create' | 'enable';

const pickModalType = (context: Pick<SignlessContext, 'pair' | 'isSessionActive'>): ModalType =>
  context.isSessionActive && !context.pair ? 'enable' : 'create';

const toModalConfig = (
  type: ModalType,
  options: AutoSignlessOptions,
  contextAllowedActions?: string[],
): SignlessSessionModalConfig => {
  if (type === 'enable') {
    return {
      type: 'enable',
      getPendingTransaction: options.getPendingTransaction,
    };
  }

  return {
    type: 'create',
    allowedActions: options.allowedActions ?? contextAllowedActions ?? [],
    shouldIssueVoucher: options.shouldIssueVoucher ?? true,
    onSessionCreate: options.onSessionCreate,
    boundSessionDuration: options.boundSessionDuration,
    getPendingTransaction: options.getPendingTransaction,
  };
};

export const useAutoSignless = (signlessContext: SignlessContext) => {
  const handleAutoSignless = useCallback(
    async (options: AutoSignlessOptions): Promise<void> => {
      if (!signlessContext.isSessionReady) {
        throw new Error('Signless session is not ready');
      }

      const modalType = pickModalType({
        pair: signlessContext.pair,
        isSessionActive: signlessContext.isSessionActive,
      });

      if (modalType === 'create' && !options.allowedActions && !signlessContext.allowedActions) {
        throw new Error('Auto signless requires allowedActions to create a session');
      }

      if (!signlessContext.pair || !signlessContext.isSessionActive) {
        const modalConfig = toModalConfig(modalType, options, signlessContext.allowedActions);
        await signlessContext.openSessionModal(modalConfig);
      }
    },
    [signlessContext],
  );

  return { handleAutoSignless };
};

export type { ModalType };
