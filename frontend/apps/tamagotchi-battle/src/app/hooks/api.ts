import { Buffer } from 'buffer';

import { getStateMetadata, ProgramMetadata, StateMetadata } from '@gear-js/api';
import { useAlert, useReadFullState } from '@gear-js/react-hooks';
import { AnyJson } from '@polkadot/types/types';
import { HexString } from '@polkadot/util/types';
import { useEffect, useState } from 'react';

export function useProgramMetadata(source: string) {
  const alert = useAlert();

  const [metadata, setMetadata] = useState<ProgramMetadata>();

  useEffect(() => {
    fetch(source)
      .then((response) => response.text())
      .then((raw) => ProgramMetadata.from(`0x${raw}`))
      .then((result) => setMetadata(result))
      .catch(({ message }: Error) => alert.error(message));

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return metadata;
}

export function useStateMetadata(source: string) {
  const alert = useAlert();

  const [data, setData] = useState<{
    buffer: Buffer;
    meta: StateMetadata;
  }>();

  useEffect(() => {
    fetch(source)
      .then((response) => response.arrayBuffer())
      .then((arrayBuffer) => Buffer.from(arrayBuffer))
      .then(async (buffer) => ({
        buffer,
        meta: await getStateMetadata(buffer),
      }))
      .then((result) => setData(result))
      .catch(({ message }: Error) => alert.error(message));

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return data;
}

export function useReadState<T>({
  programId,
  meta,
  payload,
}: {
  programId?: HexString;
  meta: string;
  payload: AnyJson;
}) {
  const metadata = useProgramMetadata(meta);
  return useReadFullState<T>(programId, metadata, payload);
}
