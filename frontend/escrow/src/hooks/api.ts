import { useSendMessageHandler, useReadWasmState } from '@gear-js/react-hooks';
import { Escrow, Wallet } from 'types';
import stateMetaWasm from 'assets/wasm/state.meta.wasm';
import metaTxt from 'assets/meta/meta.txt';
import { getProgramId } from 'utils';
import { useMetadata, useWasmMetadata } from './useMetadata';

function useEscrowMetadata() {
  return useMetadata(metaTxt);
}

function useEscrowState<T>(functionName: string, argument?: any) {
  const programMetadata = useEscrowMetadata();
  const { buffer } = useWasmMetadata(stateMetaWasm);

  return useReadWasmState<T>({
    programMetadata,
    functionName,
    argument,
    programId: getProgramId(),
    wasm: buffer,
    payload: '0x',
  });
}

function useEscrow(id: string | undefined) {
  const { state, isStateRead } = useEscrowState<Escrow>('info', id);

  const escrow = id ? state : undefined;
  const isEscrowRead = id ? isStateRead : true;

  return { escrow, isEscrowRead };
}

function useWallets(walletId: string | undefined) {
  const { state, isStateRead } = useEscrowState<Wallet[]>('created_wallets', null);

  const isWalletsStateRead = walletId ? isStateRead : true;

  return { wallets: state, isWalletsStateRead };
}

function useEscrowMessage() {
  const meta = useEscrowMetadata();
  return useSendMessageHandler(getProgramId(), meta);
}

export { useEscrow, useWallets, useEscrowMessage, useEscrowMetadata };
