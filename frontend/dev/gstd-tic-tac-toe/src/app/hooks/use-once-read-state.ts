import { HexString } from '@polkadot/util/types';
import { ProgramMetadata } from '@gear-js/api';
import { AnyJson } from '@polkadot/types/types';
import { useApi } from '@gear-js/react-hooks';
import { useState } from 'react';

export function useOnceReadFullState<T>(programId?: HexString, meta?: ProgramMetadata, payload?: AnyJson) {
  const { api } = useApi();

  const [state, setState] = useState<T>();
  const [isStateRead, setIsStateRead] = useState(false);
  const [error, setError] = useState('');

  const isPayload = payload !== undefined;

  const handleReadState = (metadata: ProgramMetadata) => {
    console.log('read state:');
    console.log(!!api);
    console.log(!!programId);
    console.log(!!metadata);
    console.log(!!isPayload);
    if (!api || !programId || !metadata || !isPayload) return;
    setIsStateRead(false);
    api.programState
      .read({ programId, payload }, metadata)
      .then((res) => res.toHuman() as T)
      .then((state) => setState(state))
      .catch((e) => setError(e))
      .finally(() => setIsStateRead(true));
  };

  return { state, isStateRead, error, handleReadState };
}

export function useOnceReadState<T>({
  programId,
  meta,
  payload,
}: {
  programId?: HexString;
  meta?: ProgramMetadata;
  payload?: AnyJson;
}) {
  return useOnceReadFullState<T>(programId, meta, payload);
}
