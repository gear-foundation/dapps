import { ReactNode, useCallback, useEffect, useMemo, useRef, useState } from 'react';

import { CreateSessionModal } from '../components/create-session-modal';
import { EnableSessionModal } from '../components/enable-session-modal';

import { SignlessTransactionsContext } from './context';
import type { SignlessContext, SignlessSessionModalConfig } from './types';

type CreateConfig = Extract<SignlessSessionModalConfig, { type: 'create' }>;
type EnableConfig = Extract<SignlessSessionModalConfig, { type: 'enable' }>;

type SignlessTransactionsModalProviderProps = {
  value: Omit<SignlessContext, 'openSessionModal'>;
  children: ReactNode;
};

type Deferred<T> = {
  promise: Promise<T>;
  resolve: (value: T) => void;
  reject: (reason?: unknown) => void;
};

type ModalState = CreateConfig | EnableConfig;

const createDeferred = <T,>(): Deferred<T> => {
  let resolve!: Deferred<T>['resolve'];
  let reject!: Deferred<T>['reject'];

  const promise = new Promise<T>((res, rej) => {
    resolve = res;
    reject = rej;
  });

  return { promise, resolve, reject };
};

const SignlessTransactionsModalProvider = ({ value, children }: SignlessTransactionsModalProviderProps) => {
  const [modalState, setModalState] = useState<ModalState | null>(null);
  const modalDeferredRef = useRef<Deferred<void> | null>(null);
  const pairRef = useRef(value.pair);
  const isSessionActiveRef = useRef(value.isSessionActive);

  useEffect(() => {
    pairRef.current = value.pair;
  }, [value.pair]);

  useEffect(() => {
    isSessionActiveRef.current = value.isSessionActive;
  }, [value.isSessionActive]);

  const handleModalClose = useCallback(() => {
    setModalState(null);

    setTimeout(() => {
      const deferred = modalDeferredRef.current;

      if (!deferred) return;

      modalDeferredRef.current = null;

      if (pairRef.current && isSessionActiveRef.current) {
        deferred.resolve();
      } else {
        deferred.reject(new Error('Signless session was not enabled'));
      }
    }, 1000);
  }, []);

  const openSessionModal = useCallback((config: SignlessSessionModalConfig) => {
    if (modalDeferredRef.current) {
      return modalDeferredRef.current.promise;
    }

    if (config.type === 'create' && config.allowedActions.length === 0) {
      throw new Error('Signless session creation requires allowedActions');
    }

    const deferred = createDeferred<void>();
    modalDeferredRef.current = deferred;

    setModalState(config);

    return deferred.promise;
  }, []);

  const contextValue = useMemo<SignlessContext>(
    () => ({
      ...value,
      openSessionModal,
    }),
    [openSessionModal, value],
  );

  return (
    <SignlessTransactionsContext.Provider value={contextValue}>
      {children}
      {modalState?.type === 'create' && (
        <CreateSessionModal
          allowedActions={modalState.allowedActions}
          close={handleModalClose}
          shouldIssueVoucher={modalState.shouldIssueVoucher}
          onSessionCreate={modalState.onSessionCreate}
          boundSessionDuration={modalState.boundSessionDuration}
        />
      )}
      {modalState?.type === 'enable' && <EnableSessionModal close={handleModalClose} />}
    </SignlessTransactionsContext.Provider>
  );
};

export { SignlessTransactionsModalProvider };
