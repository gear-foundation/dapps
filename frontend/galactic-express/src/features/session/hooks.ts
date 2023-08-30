import { useReadFullState } from '@gear-js/react-hooks';
import metaTxt from 'assets/state/launch_site.meta.txt';
import { ADDRESS } from 'consts';
import { useProgramMetadata } from 'hooks';
import { useAuth } from 'features/auth';
import { LaunchState } from './types';

function useLaunchState() {
  const meta = useProgramMetadata(metaTxt);
  const { state } = useReadFullState<LaunchState>(ADDRESS.CONTRACT, meta);

  return state;
}

function useLaunchMessage() {
  const { useSendMessage } = useAuth();
  const meta = useProgramMetadata(metaTxt);

  return useSendMessage(ADDRESS.CONTRACT, meta, { isMaxGasLimit: true });
}

export { useLaunchState, useLaunchMessage };
