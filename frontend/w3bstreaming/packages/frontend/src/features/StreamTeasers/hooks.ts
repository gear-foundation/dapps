import { useReadFullState } from '@gear-js/react-hooks';
import metaTxt from '@/assets/meta/meta.txt';
import { ADDRESS } from '@/consts';
import { useProgramMetadata } from '@/hooks';
import { Streams } from './types';

function useStreamTeasersState() {
  const programId = ADDRESS.CONTRACT;
  const meta = useProgramMetadata(metaTxt);
  const { state, isStateRead } = useReadFullState(programId, meta);

  return { streamTeasers: (state as any)?.streams as Streams, isStateRead };
}

export { useStreamTeasersState };
