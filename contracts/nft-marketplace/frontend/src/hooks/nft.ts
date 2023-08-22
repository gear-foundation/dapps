import { useAccount, useReadWasmState, useSendMessage } from '@gear-js/react-hooks';
import { AnyJson } from '@polkadot/types/types';
import { ADDRESS } from 'consts';
import { BaseNFT } from 'types';
import metaTxt from 'assets/state/nft_meta.txt';
import stateWasm from 'assets/state/nft_state.meta.wasm';
import { useBuffer, useProgramMetadata } from './metadata';

function useNftStateBuffer() {
  return useBuffer(stateWasm);
}

function useNftWasmState<T>(functionName: string, payload: AnyJson) {
  const buffer = useBuffer(stateWasm);

  return useReadWasmState<T>(ADDRESS.NFT_CONTRACT, buffer, functionName, payload);
}

function useNft(tokenId: string) {
  const { state } = useNftWasmState<BaseNFT>('token', tokenId);

  return state;
}

function useOwnersNft() {
  const { account } = useAccount();
  const { decodedAddress } = account || {};

  const { state, isStateRead } = useNftWasmState<BaseNFT[]>('tokens_for_owner', decodedAddress);

  return { NFTs: state, isEachNFTRead: isStateRead };
}

function useNftMessage() {
  const metadata = useProgramMetadata(metaTxt);

  return useSendMessage(ADDRESS.NFT_CONTRACT, metadata, { isMaxGasLimit: false });
}

export { useNftStateBuffer, useNft, useNftMessage, useOwnersNft };
