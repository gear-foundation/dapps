import { getProgramMetadata, getStateMetadata, MessagesDispatched, ProgramMetadata } from '@gear-js/api';
import { DEFAULT_ERROR_OPTIONS, DEFAULT_SUCCESS_OPTIONS, useAccount, useAlert, useApi } from '@gear-js/react-hooks';
import { web3FromSource } from '@polkadot/extension-dapp';
import { EventRecord } from '@polkadot/types/interfaces';
import { AnyJson, Codec, ISubmittableResult } from '@polkadot/types/types';
import { bnToBn } from '@polkadot/util';
import { HexString } from '@polkadot/util/types';
import { useState, useEffect, useRef } from 'react';
import stateWasm from 'assets/gear_subscription_state.meta.wasm';
import { ADDRESS } from 'consts';

const metaHex =
  '0x01000000000001050000000000000000000108000000590d4c00000004080410000410106773746418636f6d6d6f6e287072696d6974697665731c4163746f724964000004000801205b75383b2033325d000008000003200000000c000c0000050300100000050700140850676561725f737562736372697074696f6e5f696f1c416374696f6e73000110505265676973746572537562736372697074696f6e0c01387061796d656e745f6d6574686f6404011c4163746f724964000118706572696f64180118506572696f64000130776974685f72656e6577616c1c0110626f6f6c00000048557064617465537562736372697074696f6e0401287375627363726962657204011c4163746f7249640001004843616e63656c537562736372697074696f6e000200644d616e61676550656e64696e67537562736372697074696f6e040118656e61626c651c0110626f6f6c00030000180850676561725f737562736372697074696f6e5f696f18506572696f640001141059656172000000284e696e654d6f6e746873000100245369784d6f6e7468730002002c54687265654d6f6e746873000300144d6f6e7468000400001c0000050000200850676561725f737562736372697074696f6e5f696f44537562736372697074696f6e5374617465000008012c737562736372696265727324018442547265654d61703c4163746f7249642c2053756273637269626572446174613e00013c7061796d656e745f6d6574686f647344016042547265654d61703c4163746f7249642c2050726963653e000024042042547265654d617008044b0104045601280004003c000000280850676561725f737562736372697074696f6e5f696f38537562736372696265724461746100001001387061796d656e745f6d6574686f6404011c4163746f724964000118706572696f64180118506572696f64000148737562736372697074696f6e5f73746172742c01484f7074696f6e3c287536342c20753332293e00013072656e6577616c5f646174652c01484f7074696f6e3c287536342c20753332293e00002c04184f7074696f6e04045401300108104e6f6e6500000010536f6d65040030000001000030000004083438003400000506003800000505003c0000024000400000040804280044042042547265654d617008044b01040456011000040048000000480000020000' as HexString;

const ftMetaHex =
  '0x0100000000000103000000010700000000000000000108000000a90b3400081466745f696f28496e6974436f6e66696700000c01106e616d65040118537472696e6700011873796d626f6c040118537472696e67000120646563696d616c73080108753800000400000502000800000503000c081466745f696f204654416374696f6e000118104d696e74040010011075313238000000104275726e040010011075313238000100205472616e736665720c011066726f6d14011c4163746f724964000108746f14011c4163746f724964000118616d6f756e74100110753132380002001c417070726f7665080108746f14011c4163746f724964000118616d6f756e74100110753132380003002c546f74616c537570706c790004002442616c616e63654f66040014011c4163746f724964000500001000000507001410106773746418636f6d6d6f6e287072696d6974697665731c4163746f724964000004001801205b75383b2033325d0000180000032000000008001c081466745f696f1c46544576656e74000110205472616e736665720c011066726f6d14011c4163746f724964000108746f14011c4163746f724964000118616d6f756e74100110753132380000001c417070726f76650c011066726f6d14011c4163746f724964000108746f14011c4163746f724964000118616d6f756e74100110753132380001002c546f74616c537570706c790400100110753132380002001c42616c616e63650400100110753132380003000020081466745f696f3c496f46756e6769626c65546f6b656e00001801106e616d65040118537472696e6700011873796d626f6c040118537472696e67000130746f74616c5f737570706c791001107531323800012062616c616e6365732401505665633c284163746f7249642c2075313238293e000128616c6c6f77616e6365732c01905665633c284163746f7249642c205665633c284163746f7249642c2075313238293e293e000120646563696d616c730801087538000024000002280028000004081410002c00000230003000000408142400' as HexString;

const programId = '0x95e4b710736a53f4917194cc1a5122df7cf4b6e9d11652470a53a8cdc1ffe296' as HexString;

const metadata = getProgramMetadata(metaHex);

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

  const { state, isStateRead } = useReadWasmState<FullSubState>(programId, buffer, 'all_subscriptions');

  return { subscriptionsState: state, isSubscriptionsStateRead: isStateRead };
}

function useSubscriptionsMessage() {
  return useSendMessage(programId, metadata);
}

type FTState = { balances: [[HexString, number]] };

function useFTBalance() {
  const { account } = useAccount();
  const { decodedAddress } = account || {};

  const meta = getProgramMetadata(ftMetaHex);
  const { state, isStateRead } = useReadFullState<FTState>(ADDRESS.FT_CONTRACT, meta);

  const balances = state?.balances;
  const userBalanceEntity = balances?.find(([address]) => address === decodedAddress);
  const [, balance] = userBalanceEntity || [];

  return balance;
}

export { useSubscriptions, useSubscriptionsMessage, useFTBalance };
