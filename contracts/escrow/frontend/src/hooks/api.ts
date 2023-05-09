import { useSendMessage, useReadWasmState } from '@gear-js/react-hooks';
import { Escrow, Wallet } from 'types';
import stateMetaWasm from 'assets/wasm/state.meta.wasm';
import metaTxt from 'assets/meta/meta.txt';
import { getProgramId } from 'utils';
import { useMetadata, useWasmMetadata } from './useMetadata';

function useEscrowMetadata() {
  return useMetadata(metaTxt);
}

function useEscrowState<T>(functionName: string, payload?: any) {
  const { buffer } = useWasmMetadata(stateMetaWasm);

  return useReadWasmState<T>(getProgramId(), buffer, functionName, payload);
}

function useEscrow(id: string | undefined) {
  const { state, isStateRead } = useEscrowState<Escrow>('info', id);

  const escrow = id ? state : undefined;
  const isEscrowRead = id ? isStateRead : true;

  return { escrow, isEscrowRead };
}

function useWallets(walletId: string | undefined) {
  const { state, isStateRead } = useEscrowState<Wallet[]>(
    'created_wallets',
    null
  );

  const isWalletsStateRead = walletId ? isStateRead : true;

  return { wallets: state, isWalletsStateRead };
}

function useEscrowMessage() {
  const meta = useEscrowMetadata();
  return useSendMessage(getProgramId(), meta);
}

export { useEscrow, useWallets, useEscrowMessage, useEscrowMetadata };
