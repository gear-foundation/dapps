import { useCallback, useRef, useEffect } from 'react';

import type { SignlessContext, SignlessSessionModalConfig } from '../context/types';

export type AutoSignlessOptions = {
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

export const useAutoSignless = (signlessContext: SignlessContext, defaultOptions?: AutoSignlessOptions) => {
  const defaultAutoSignlessRef = useRef<ResolvedAutoSignlessOptions>({
    allowedActions: defaultOptions?.allowedActions ?? [],
    shouldIssueVoucher: defaultOptions?.shouldIssueVoucher ?? true,
    onSessionCreate: defaultOptions?.onSessionCreate,
    boundSessionDuration: defaultOptions?.boundSessionDuration,
  });

  useEffect(() => {
    defaultAutoSignlessRef.current = {
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

  const handleAutoSignless = useCallback(
    async (overrides?: AutoSignlessOptions): Promise<void> => {
      if (!signlessContext.isSessionReady) {
        throw new Error('Signless session is not ready');
      }

      const resolvedOptions = getResolvedOptions(defaultAutoSignlessRef.current, overrides);
      const modalType = pickModalType({
        pair: signlessContext.pair,
        isSessionActive: signlessContext.isSessionActive,
      });

      if (modalType === 'create' && resolvedOptions.allowedActions.length === 0) {
        throw new Error('Auto signless requires allowedActions to create a session');
      }

      if (!signlessContext.pair || !signlessContext.isSessionActive) {
        const modalConfig = toModalConfig(modalType, resolvedOptions);
        await signlessContext.openSessionModal(modalConfig);
      }
    },
    [signlessContext],
  );

  return { handleAutoSignless };
};

export type { ResolvedAutoSignlessOptions, ModalType };
