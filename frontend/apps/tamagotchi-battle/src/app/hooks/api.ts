import { ProgramMetadata } from '@gear-js/api';
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
