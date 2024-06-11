import { useAccount, useReadWasmState, useSendMessageWithGas } from '@gear-js/react-hooks';
import { AnyJson } from '@polkadot/types/types';
import { ADDRESS } from 'consts';
import { BaseNFT } from 'types';
import metaTxt from 'assets/state/nft_meta.txt';
import stateWasm from 'assets/state/nft_state.meta.wasm';
import { useBuffer, useProgramMetadata } from './metadata';

function useNftMeta() {
  return useProgramMetadata(metaTxt);
}

function useNftStateBuffer() {
  return useBuffer(stateWasm);
}

function useNftWasmState<T>(functionName: string, argument: AnyJson) {
  const programMetadata = useNftMeta();
  const buffer = useBuffer(stateWasm);

  return useReadWasmState<T>({
    programId: ADDRESS.NFT_CONTRACT,
    wasm: buffer,
    functionName,
    payload: '0x',
    argument,
    programMetadata,
  });
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
  const metadata = useNftMeta();

  return useSendMessageWithGas(ADDRESS.NFT_CONTRACT, metadata);
}

export { useNftStateBuffer, useNft, useNftMessage, useOwnersNft, useNftMeta };
