import metaTxt from 'assets/state/launch_site.meta.txt';
import { useProgramMetadata } from 'hooks';

function useEscrowMetadata() {
  return useProgramMetadata(metaTxt);
}

export { useEscrowMetadata };
