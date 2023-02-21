import { getProgramMetadata, getStateMetadata, MessagesDispatched, ProgramMetadata } from '@gear-js/api';
import { DEFAULT_ERROR_OPTIONS, DEFAULT_SUCCESS_OPTIONS, useAccount, useAlert, useApi } from '@gear-js/react-hooks';
import { web3FromSource } from '@polkadot/extension-dapp';
import { EventRecord } from '@polkadot/types/interfaces';
import { AnyJson, Codec, ISubmittableResult } from '@polkadot/types/types';
import { bnToBn } from '@polkadot/util';
import { HexString } from '@polkadot/util/types';
import { useState, useEffect, useRef } from 'react';
import stateWasm from 'assets/gear_subscription_state.meta.wasm';
import { ADDRESS, META_HEX } from 'consts';

const metadata = getProgramMetadata(META_HEX.SUBSCRIPTION);

type FullSubState = {
  [key: HexString]: {
    isActive: boolean;
    startDate: number;
    endDate: number;
    startBlock: number;
    endBlock: number;
    period: string;
    renewalDate: number;
    renewalBlock: number;
    price: number;
    willRenew: boolean;
  };
};

function useHandleReadState<T = AnyJson>(
  handleReadState: () => Promise<Codec> | undefined,
  isReadOnError: boolean | undefined,
) {
  const alert = useAlert();

  const [state, setState] = useState<T>();
  const [error, setError] = useState('');
  const [isStateRead, setIsStateRead] = useState(false);

  console.log(isStateRead);
  console.log(state);

  const resetError = () => setError('');

  const readState = (isInitLoad?: boolean) => {
    if (isInitLoad) setIsStateRead(false);

    handleReadState()
      ?.then((codecState) => codecState.toJSON())
      .then((result) => {
        setState(result as unknown as T);
        if (!isReadOnError) setIsStateRead(true);
      })
      .catch(({ message }: Error) => setError(message))
      .finally(() => {
        if (isReadOnError) setIsStateRead(true);
      });
  };

  useEffect(() => {
    if (error) alert.error(error);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [error]);

  return { state, isStateRead, error, readState, resetError };
}

// eslint-disable-next-line @typescript-eslint/no-shadow
function useStateSubscription(programId: HexString | undefined, onStateChange: () => void, dependency?: boolean) {
  const { api } = useApi(); // сircular dependency fix

  const handleStateChange = ({ data }: MessagesDispatched) => {
    const changedIDs = data.stateChanges.toHuman() as HexString[];
    const isAnyChange = changedIDs.some((id) => id === programId);

    if (isAnyChange) onStateChange();
  };

  useEffect(() => {
    const isDependency = dependency !== undefined;

    if (!programId || (isDependency && !dependency)) return;

    const unsub = api?.gearEvents.subscribeToGearEvent('MessagesDispatched', handleStateChange);

    return () => {
      unsub?.then((unsubCallback: any) => unsubCallback());
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [api, programId, dependency]);
}

function useReadFullState<T = AnyJson>(
  // eslint-disable-next-line @typescript-eslint/no-shadow
  programId: HexString | undefined,
  meta: ProgramMetadata | undefined,
  isReadOnError?: boolean,
) {
  const { api } = useApi(); // сircular dependency fix

  const readFullState = () => {
    if (!programId || !meta) return;

    return api.programState.read({ programId }, meta);
  };

  const { state, isStateRead, error, readState, resetError } = useHandleReadState<T>(readFullState, isReadOnError);

  useEffect(() => {
    readState(true);
    resetError();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [programId, meta]);

  useStateSubscription(programId, readState, !!meta);

  return { state, isStateRead, error };
}

type SendMessageOptions = {
  value?: string | number;
  isOtherPanicsAllowed?: boolean;
  onSuccess?: () => void;
  onError?: () => void;
};

// eslint-disable-next-line @typescript-eslint/no-shadow
function useSendMessage(destination: HexString, metadata: ProgramMetadata | undefined) {
  const { api } = useApi(); // сircular dependency fix
  const { account } = useAccount();
  const alert = useAlert();

  const title = 'gear.sendMessage';
  const loadingAlertId = useRef('');

  const handleEventsStatus = (events: EventRecord[], onSuccess?: () => void, onError?: () => void) => {
    events.forEach(({ event: { method, section } }) => {
      if (method === 'MessageEnqueued') {
        alert.success(`${section}.MessageEnqueued`);
        // eslint-disable-next-line @typescript-eslint/no-unused-expressions
        onSuccess && onSuccess();
      } else if (method === 'ExtrinsicFailed') {
        alert.error('Extrinsic Failed', { title });
        // eslint-disable-next-line @typescript-eslint/no-unused-expressions
        onError && onError();
      }
    });
  };

  const handleStatus = (result: ISubmittableResult, onSuccess?: () => void, onError?: () => void) => {
    const { status, events } = result;
    const { isReady, isInBlock, isInvalid, isFinalized } = status;

    if (isInvalid) {
      alert.update(loadingAlertId.current, 'Transaction error. Status: isInvalid', DEFAULT_ERROR_OPTIONS);
    } else if (isReady) {
      alert.update(loadingAlertId.current, 'Ready');
    } else if (isInBlock) {
      alert.update(loadingAlertId.current, 'In Block');
    } else if (isFinalized) {
      alert.update(loadingAlertId.current, 'Finalized', DEFAULT_SUCCESS_OPTIONS);
      handleEventsStatus(events, onSuccess, onError);
    }
  };

  const sendMessage = (payload: AnyJson, options?: SendMessageOptions) => {
    if (account && metadata) {
      loadingAlertId.current = alert.loading('Sign In', { title });

      const { value = 0, isOtherPanicsAllowed = false, onSuccess, onError } = options || {};
      const { address, decodedAddress, meta } = account;
      const { source } = meta;

      // eslint-disable-next-line @typescript-eslint/no-unused-expressions
      api.message.send({ destination, gasLimit: bnToBn('250000000000'), payload, value }, metadata) &&
        web3FromSource(source)
          .then(({ signer }) =>
            api.message.signAndSend(address, { signer }, (result) => handleStatus(result, onSuccess, onError)),
          )
          .catch(({ message }: Error) => {
            alert.update(loadingAlertId.current, message, DEFAULT_ERROR_OPTIONS);
            // eslint-disable-next-line @typescript-eslint/no-unused-expressions
            onError && onError();
          });
    }
  };

  return sendMessage;
}
function useReadWasmState<T = AnyJson>(
  // eslint-disable-next-line @typescript-eslint/no-shadow
  programId: HexString | undefined,
  wasm: Buffer | Uint8Array | undefined,
  functionName: string | undefined,
  payload?: AnyJson,
  isReadOnError?: boolean,
) {
  const { api } = useApi();

  const readWasmState = () => {
    if (!programId || !wasm || !functionName) return;

    return getStateMetadata(wasm).then((stateMetadata) =>
      api.programState.readUsingWasm({ programId, wasm, fn_name: functionName, argument: payload }, stateMetadata),
    );
  };

  const alert = useAlert();

  const [state, setState] = useState<T>();
  const [error, setError] = useState('');
  const [isStateRead, setIsStateRead] = useState(false);

  const resetError = () => setError('');

  const readState = (isInitLoad?: boolean) => {
    if (isInitLoad) setIsStateRead(false);

    readWasmState()
      ?.then((codecState) => codecState.toJSON())
      .then((result) => {
        setState(result as unknown as T);
        if (!isReadOnError) setIsStateRead(true);
      })
      .catch(({ message }: Error) => setError(message))
      .finally(() => {
        if (isReadOnError) setIsStateRead(true);
      });
  };

  useEffect(() => {
    if (error) alert.error(error);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [error]);

  useEffect(() => {
    readState(true);
    resetError();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [programId, wasm, functionName]);

  const handleStateChange = ({ data }: MessagesDispatched) => {
    const changedIDs = data.stateChanges.toHuman() as HexString[];
    const isAnyChange = changedIDs.some((id) => id === programId);

    if (isAnyChange) readState();
  };

  useEffect(() => {
    if (!programId || !wasm || !functionName) return;

    const unsub = api?.gearEvents.subscribeToGearEvent('MessagesDispatched', handleStateChange);

    return () => {
      unsub?.then((unsubCallback) => unsubCallback());
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [api, programId, wasm, functionName]);

  return { state, isStateRead, error };
}

function useSubscriptions() {
  const [buffer, setBuffer] = useState<Buffer>();

  useEffect(() => {
    fetch(stateWasm)
      .then((result) => result.arrayBuffer())
      .then((arrBuffer) => Buffer.from(arrBuffer))
      .then((res) => setBuffer(res));
  }, []);

  const { state, isStateRead } = useReadWasmState<FullSubState>(ADDRESS.CONTRACT, buffer, 'all_subscriptions');

  return { subscriptionsState: state, isSubscriptionsStateRead: isStateRead };
}

function useSubscriptionsMessage() {
  return useSendMessage(ADDRESS.CONTRACT, metadata);
}

type FTState = { balances: [[HexString, number]] };

function useFTBalance() {
  const { account } = useAccount();
  const { decodedAddress } = account || {};

  const meta = getProgramMetadata(META_HEX.FT);
  const { state, isStateRead } = useReadFullState<FTState>(ADDRESS.FT_CONTRACT, meta);

  const balances = state?.balances;
  const userBalanceEntity = balances?.find(([address]) => address === decodedAddress);
  const [, balance] = userBalanceEntity || [];

  return balance;
}

export { useSubscriptions, useSubscriptionsMessage, useFTBalance };
