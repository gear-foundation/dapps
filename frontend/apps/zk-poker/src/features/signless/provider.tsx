import { EnableSessionModal } from '@ez/features/signless-transactions/components/enable-session-modal';
import { ProviderProps } from '@gear-js/react-hooks';
import { GenericTransactionReturn, TransactionReturn } from '@gear-js/react-hooks/dist/hooks/sails/types';
import { CreateSessionModal, SignlessContext, useEzTransactions, useCreateSailsSession } from 'gear-ez-transactions';
import { createContext, useContext, useState, useCallback } from 'react';

import { SIGNLESS_ALLOWED_ACTIONS } from '@/app/consts';
import { usePokerProgram } from '@/app/utils';

interface TransactionWithSessionOptions {
  onSuccess?: () => void;
  onError?: (error: Error) => void;
  onFinally?: () => void;
}

type Transaction = TransactionReturn<(...args: unknown[]) => GenericTransactionReturn<null>>;

interface AutoSignlessContextType {
  executeWithSessionModal: (transaction: Transaction, sessionForAccount: `0x${string}` | null) => Promise<void>;
  closeModal: () => void;
}

const AutoSignlessContext = createContext<AutoSignlessContextType | undefined>(undefined);

const AutoSignlessProvider = ({ children }: ProviderProps) => {
  const [modalOpen, setModalOpen] = useState<'create-session' | 'enable-session' | null>(null);
  const [pendingTransaction, setPendingTransaction] = useState<Transaction | null>(null);
  const [pendingOptions, setPendingOptions] = useState<TransactionWithSessionOptions>({});
  const { gasless, signless } = useEzTransactions();
  const program = usePokerProgram();
  const { createSession } = useCreateSailsSession(program?.programId || '0x', program);

  const openModalWithExtrinsic = useCallback(
    (
      transaction: Transaction,
      modal: 'create-session' | 'enable-session',
      options: TransactionWithSessionOptions = {},
    ) => {
      setPendingTransaction(transaction);
      setPendingOptions(options);
      setModalOpen(modal);
    },
    [],
  );

  const executeWithSessionModal = useCallback(
    async (transaction: Transaction, sessionForAccount: `0x${string}` | null) => {
      console.log('🚀 ~ signless:', signless);
      if (sessionForAccount) {
        await transaction.signAndSend();
        return;
      } else {
        const modal = signless.session && signless.storagePair && !signless.pair ? 'enable-session' : 'create-session';
        openModalWithExtrinsic(transaction, modal);
      }
    },
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [signless],
  );

  const closeModal = useCallback(() => {
    setModalOpen(null);
    setPendingTransaction(null);
    setPendingOptions({});
  }, []);

  const useCustomSignlessContext = useCallback(
    (): SignlessContext => ({
      ...signless,
      createSession: (...createSessionParams) => {
        if (pendingTransaction) {
          const [session, voucherValue, params] = createSessionParams;

          void createSession(session, voucherValue, {
            ...params,
            additionalExtrinsics: [pendingTransaction.extrinsic],
            onSuccess: () => {
              pendingOptions.onSuccess?.();
              params.onSuccess?.();
              closeModal();
            },
          });
        }
      },
    }),
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [signless, pendingOptions, closeModal, pendingTransaction],
  );

  const contextValue: AutoSignlessContextType = {
    executeWithSessionModal,
    closeModal,
  };

  return (
    <AutoSignlessContext.Provider value={contextValue}>
      <>
        {children}

        {modalOpen === 'create-session' && (
          <CreateSessionModal
            allowedActions={SIGNLESS_ALLOWED_ACTIONS}
            close={closeModal}
            onSessionCreate={signless.onSessionCreate}
            shouldIssueVoucher={!gasless.isEnabled}
            useCustomSignlessContext={useCustomSignlessContext}
            boundSessionDuration={gasless.isEnabled ? gasless.voucherStatus?.duration : undefined}
            maxWidth="375px"
          />
        )}

        {modalOpen === 'enable-session' && <EnableSessionModal close={() => closeModal()} maxWidth="375px" />}
      </>
    </AutoSignlessContext.Provider>
  );
};

const useAutoSignless = (): AutoSignlessContextType => {
  const context = useContext(AutoSignlessContext);

  if (!context) {
    throw new Error('useTransactionWithSessionModal must be used within AutoSignlessProvider');
  }
  return context;
};

export { AutoSignlessProvider, useAutoSignless };
