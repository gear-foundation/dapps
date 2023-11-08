import metaTxt from 'assets/meta/galactic_express_meta.txt';
import { useProgramMetadata } from 'hooks';

function useEscrowMetadata() {
  return useProgramMetadata(metaTxt);
}

export { useEscrowMetadata };
