import { useReadFullState } from '@gear-js/react-hooks';
import { ADDRESS } from '@/consts';
import { Streams } from './types';
import { useGetStreamMetadata } from '../CreateStream/hooks';

function useStreamTeasersState() {
  const programId = ADDRESS.CONTRACT;
  const { meta } = useGetStreamMetadata();
  const { state, isStateRead } = useReadFullState(programId, meta, '0x');

  return { streamTeasers: (state as any)?.streams as Streams, isStateRead };
}

export { useStreamTeasersState };
