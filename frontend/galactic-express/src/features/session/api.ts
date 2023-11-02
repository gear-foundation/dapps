import metaTxt from 'assets/meta/galactic_express.meta.txt';
import { useProgramMetadata } from 'hooks';

function useEscrowMetadata() {
  return useProgramMetadata(metaTxt);
}

export { useEscrowMetadata };
