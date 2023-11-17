import { useCreateHandler } from '@gear-js/react-hooks';
import { HexString } from '@polkadot/util/types';
import { ADDRESS } from 'consts';
import { InitPayload } from 'types';
import { useSupplyChainMetadata } from './api';

function useCreateSupplyChain(onSuccess: (programId: HexString) => void) {
  const metadata = useSupplyChainMetadata();

  const createProgram = useCreateHandler(ADDRESS.CODE, metadata);

  return (payload: InitPayload) => createProgram(payload, { onSuccess });
}

export { useCreateSupplyChain };
