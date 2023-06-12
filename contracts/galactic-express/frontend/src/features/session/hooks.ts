import { useReadFullState } from '@gear-js/react-hooks';
import metaTxt from 'assets/state/launch_site.meta.txt';
import { ADDRESS } from 'consts';
import { useProgramMetadata } from 'hooks';
import { SessionState } from './types';

function useSessionState() {
  const meta = useProgramMetadata(metaTxt);
  const { state } = useReadFullState<SessionState>(ADDRESS.CONTRACT, meta);

  return state;
}

export { useSessionState };
