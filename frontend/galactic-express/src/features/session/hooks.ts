import { useReadFullState, useSendMessageHandler } from '@gear-js/react-hooks';
import { HexString } from '@gear-js/api';
import metaTxt from 'assets/meta/galactic_express.meta.txt';
import { useProgramMetadata } from 'hooks';
import { LaunchState } from './types';

function useNewSessionMessage(address: string) {
  const meta = useProgramMetadata(metaTxt);

  return { meta: !!meta, message: useSendMessageHandler(address as HexString, meta, { isMaxGasLimit: true }) };
}

function useLaunchState(address: string) {
  const meta = useProgramMetadata(metaTxt);
  const { state } = useReadFullState<LaunchState>(address as HexString, meta, '0x');

  return state;
}

function useLaunchMessage(address: string) {
  const meta = useProgramMetadata(metaTxt);

  return { meta, message: useSendMessageHandler(address as HexString, meta, { isMaxGasLimit: true }) };
}

export { useLaunchState, useLaunchMessage, useNewSessionMessage };
