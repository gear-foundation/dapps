import { useAtom } from 'jotai';
import { CONTRACT_ADDRESS_ATOM } from './consts';

export function useContractAddress() {
  const [contractAddress, setContractAddress] = useAtom(CONTRACT_ADDRESS_ATOM);

  return { contractAddress, setContractAddress };
}
