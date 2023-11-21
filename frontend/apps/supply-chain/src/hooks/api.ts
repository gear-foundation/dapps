import { SendMessageOptions, useAccount, useReadWasmState, useSendMessageHandler } from '@gear-js/react-hooks';
import { AnyJson } from '@polkadot/types/types';
import { HexString } from '@polkadot/util/types';
import { LOCAL_STORAGE } from 'consts';
import { Item, Items, Token } from 'types';
import meta from 'assets/state/supply_chain_meta.txt';
import nftMeta from 'assets/state/non_fungible_token.meta.txt';
import stateWasm from 'assets/state/supply_chain_state.meta.wasm';
import nftStateWasm from 'assets/state/nft_state.meta.wasm';
import { useBuffer, useProgramMetadata } from './metadata';

function useSupplyChainMetadata() {
  return useProgramMetadata(meta);
}

function useSupplyChainState<T>(functionName: string, argument: AnyJson) {
  const programMetadata = useSupplyChainMetadata();
  const wasm = useBuffer(stateWasm);

  const programId = localStorage[LOCAL_STORAGE.PROGRAM];
  const payload = '0x';

  return useReadWasmState<T>({ programId, programMetadata, wasm, argument, payload, functionName });
}

function useItem(itemId: string) {
  const payload = itemId || undefined;
  const { state, isStateRead } = useSupplyChainState<Item>('item_info', payload);

  return { item: state, isItemRead: isStateRead };
}

function useItems() {
  const { state, isStateRead } = useSupplyChainState<Items>('existing_items', null);

  return { items: state, isEachItemRead: isStateRead };
}

function useRoles() {
  const { account } = useAccount();
  const { decodedAddress } = account || {};

  const { state, isStateRead } = useSupplyChainState<string[]>('roles', decodedAddress);

  return { roles: state, isEachRoleRead: isStateRead };
}

function useNftProgramId() {
  const { state } = useSupplyChainState<HexString>('non_fungible_token', null);

  return state;
}

function useNft(tokenId: string) {
  const programId = useNftProgramId();
  const programMetadata = useProgramMetadata(nftMeta);
  const wasm = useBuffer(nftStateWasm);

  const functionName = 'token';
  const argument = tokenId || undefined;
  const payload = '0x';

  const { state, isStateRead } = useReadWasmState<Token>({
    programId,
    programMetadata,
    wasm,
    functionName,
    argument,
    payload,
  });

  return { nft: state, isNftRead: isStateRead };
}

function useSupplyChainMessage() {
  const metadata = useSupplyChainMetadata();
  const sendMessage = useSendMessageHandler(localStorage[LOCAL_STORAGE.PROGRAM], metadata);

  return (payload: AnyJson, options?: Pick<SendMessageOptions, 'onSuccess' | 'onError'>) =>
    sendMessage({ payload: { action: payload, kind: { New: null } }, ...options });
}

export { useItem, useItems, useRoles, useNft, useSupplyChainMessage, useSupplyChainMetadata };
