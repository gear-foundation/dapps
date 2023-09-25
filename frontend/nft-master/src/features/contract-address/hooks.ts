import { useAtom } from 'jotai';
import { CONTRACT_ADDRESS_ATOM, TESTNET_CONTRACT_ADDRESS_ATOM } from './consts';
import { useNodeAddress } from '../node-switch';

export function useContractAddress() {
  const { isTestnet } = useNodeAddress();
  const [contractAddress, setContractAddress] = useAtom(
    isTestnet ? TESTNET_CONTRACT_ADDRESS_ATOM : CONTRACT_ADDRESS_ATOM,
  );
  return { contractAddress, setContractAddress };
}
