import { HexString } from '@polkadot/util/types';
import { ADDRESS } from 'consts';
import { useCreateHandler } from '@gear-js/react-hooks';
import { AnyJson } from '@polkadot/types/types';
import { useEscrowMetadata } from './api'
import { useState } from 'react';

function useProgram() {
  const meta = useEscrowMetadata();
  const createProgram = useCreateHandler(ADDRESS.CODE_ADDRESS, meta);

  const [programId, setProgramId] = useState('' as HexString);

  const resetProgramId = () => setProgramId('' as HexString);

  return {
    createProgram: (payload: AnyJson) => createProgram(payload, { onSuccess: setProgramId }),
    programId,
    setProgramId,
    resetProgramId,
  };
}

export { useProgram };
