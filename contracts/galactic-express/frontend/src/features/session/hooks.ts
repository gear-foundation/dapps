import { useReadFullState, useSendMessage } from '@gear-js/react-hooks';
import metaTxt from 'assets/state/launch_site.meta.txt';
import { ADDRESS } from 'consts';
import { useProgramMetadata } from 'hooks';
import { LaunchState } from './types';

function useLaunchState() {
  const meta = useProgramMetadata(metaTxt);
  const { state } = useReadFullState<LaunchState>(ADDRESS.CONTRACT, meta);

  return state;
}

function useLaunchMessage() {
  const meta = useProgramMetadata(metaTxt);

  return useSendMessage(ADDRESS.CONTRACT, meta);
}

export { useLaunchState, useLaunchMessage };
