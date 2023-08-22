import { Hex, MessagesDispatched } from '@gear-js/api';
import { useAlert, useApi } from '@gear-js/react-hooks';
import { AnyJson } from '@polkadot/types/types';
import { UnsubscribePromise } from '@polkadot/api/types';
import { useEffect, useState } from 'react';

type State<T> = { state: T | undefined; isStateRead: boolean; error: string };

function useReadState<T = AnyJson>(
  programId: Hex | undefined,
  metaBuffer: Buffer,
  payload?: AnyJson,
  isReadOnError?: boolean,
): State<T> {
  const { api } = useApi();
  const alert = useAlert();

  const [state, setState] = useState<T>();
  const [error, setError] = useState('');
  const [isStateRead, setIsStateRead] = useState(true);

  const readState = (isInitLoad?: boolean) => {
    if (programId && metaBuffer) {
      if (isInitLoad) setIsStateRead(false);

      api.programState
        .read(programId, metaBuffer, payload)
        .then((codecState) => codecState.toHuman())
        .then((result) => {
          setState(result as unknown as T);
          if (!isReadOnError) setIsStateRead(true);
        })
        .catch(({ message }: Error) => setError(message))
        .finally(() => {
          if (isReadOnError) setIsStateRead(true);
        });
    }
  };

  useEffect(() => {
    readState(true);
    setError('');
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [programId, metaBuffer, payload]);

  const handleStateChange = ({ data }: MessagesDispatched) => {
    const changedIDs = data.stateChanges.toHuman() as Hex[];
    const isAnyChange = changedIDs.some((id) => id === programId);

    if (isAnyChange) readState();
  };

  useEffect(() => {
    let unsub: UnsubscribePromise | undefined;

    if (api && programId && metaBuffer) {
      unsub = api.gearEvents.subscribeToGearEvent('MessagesDispatched', handleStateChange);
    }

    return () => {
      if (unsub) unsub.then((unsubCallback) => unsubCallback());
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [api, programId, metaBuffer]);

  useEffect(() => {
    if (error) alert.error(error);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [error]);

  return { state, isStateRead, error };
}

export { useReadState };
