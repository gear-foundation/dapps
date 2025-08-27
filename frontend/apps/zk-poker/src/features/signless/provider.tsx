import { EnableSessionModal } from '@ez/features/signless-transactions/components/enable-session-modal';
import { HexString } from '@gear-js/api';
import { ProviderProps, useAccount } from '@gear-js/react-hooks';
import { IKeyringPair } from '@polkadot/types/types';
import {
  CreateSessionModal,
  SignlessContext,
  useEzTransactions,
  useCreateSailsSession,
  PrepareEzTransactionParamsResult,
} from 'gear-ez-transactions';
import { useState, useCallback } from 'react';

import { SIGNLESS_ALLOWED_ACTIONS } from '@/app/consts';
import { usePokerProgram } from '@/app/utils';

import {
  AutoSignlessContext,
  AutoSignlessContextType,
  PrepareTransactionAsyncResult,
  TransactionWithSessionOptions,
} from './context';

const AutoSignlessProvider = ({ children }: ProviderProps) => {
  const [modalOpen, setModalOpen] = useState<'create-session' | 'enable-session' | null>(null);
  const [getPendingTransaction, setGetPendingTransaction] = useState<
    ((params?: Partial<PrepareEzTransactionParamsResult>) => PrepareTransactionAsyncResult) | null
  >(null);
  const [pendingOptions, setPendingOptions] = useState<TransactionWithSessionOptions>({});
  const { gasless, signless } = useEzTransactions();
  const program = usePokerProgram();
  const { createSession } = useCreateSailsSession(program?.programId || '0x', program);
  const { account } = useAccount();

  const openModalWithExtrinsic = useCallback(
    (
      getTransaction: (params?: Partial<PrepareEzTransactionParamsResult>) => PrepareTransactionAsyncResult,
      modal: 'create-session' | 'enable-session',
      options: TransactionWithSessionOptions = {},
    ) => {
      setGetPendingTransaction(() => getTransaction);
      setPendingOptions(options);
      setModalOpen(modal);
    },
    [],
  );

  const executeWithSessionModal = useCallback(
    async (
      getTransaction: (params?: Partial<PrepareEzTransactionParamsResult>) => PrepareTransactionAsyncResult,
      sessionForAccount: HexString | null,
      options: TransactionWithSessionOptions = {},
    ) => {
      if (sessionForAccount) {
        const { transaction } = await getTransaction({ sessionForAccount });
        await transaction.signAndSend();
        return;
      } else {
        const modal = signless.session && signless.storagePair && !signless.pair ? 'enable-session' : 'create-session';
        openModalWithExtrinsic(getTransaction, modal, options);
      }
    },
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [signless],
  );

  const closeModal = useCallback(() => {
    setModalOpen(null);
    setGetPendingTransaction(null);
    setPendingOptions({});
  }, []);

  const useCustomSignlessContext = useCallback(
    (): SignlessContext => ({
      ...signless,
      createSession: (...createSessionParams) => {
        if (getPendingTransaction) {
          const [session, voucherValue, params] = createSessionParams;

          const onSuccess = () => {
            pendingOptions.onSuccess?.();
            params.onSuccess?.();
            closeModal();
          };

          const onError = (error: Error) => {
            pendingOptions.onError?.(error);
            params.onError?.(error);
          };

          const onFinally = () => {
            pendingOptions.onFinally?.();
            params.onFinally?.();
          };

          getPendingTransaction()
            .then(({ transaction }) => {
              return createSession(session, voucherValue, {
                ...params,
                additionalExtrinsics: [transaction.extrinsic],
                onSuccess,
                onError,
                onFinally,
              });
            })
            .catch(onError)
            .finally(onFinally);
        }
      },
    }),
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [signless, pendingOptions, closeModal, getPendingTransaction],
  );

  const enableSessionCallback = useCallback(
    async (pair: IKeyringPair) => {
      const voucherId = signless.storageVoucher?.id;
      if (getPendingTransaction && voucherId && account) {
        const { transaction } = await getPendingTransaction({
          account: { addressOrPair: pair },
          sessionForAccount: account.decodedAddress,
          voucherId,
        });
        await transaction.signAndSend();
      }
    },
    [getPendingTransaction, signless.storageVoucher, account],
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

        {modalOpen === 'enable-session' && (
          <EnableSessionModal callback={enableSessionCallback} close={() => closeModal()} maxWidth="375px" />
        )}
      </>
    </AutoSignlessContext.Provider>
  );
};

export { AutoSignlessProvider };
