import { useReadFullState } from '@gear-js/react-hooks';
import { HexString } from '@polkadot/util/types';
import { AnyJson } from '@polkadot/types/types';

import { useProgramMetadata } from '@dapps-frontend/hooks';

export function useReadState<T>({
  programId,
  meta,
  payload,
}: {
  programId?: HexString;
  meta: string;
  payload?: AnyJson;
}) {
  const metadata = useProgramMetadata(meta);

  return useReadFullState<T>(programId, metadata, payload);
}
