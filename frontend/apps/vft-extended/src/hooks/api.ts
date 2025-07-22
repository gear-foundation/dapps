/**
 * ---------------------------------------------------------
 *  SailsJS Hooks for Foundable Token Extended (VFT) Contract
 * ---------------------------------------------------------
 *
 * This is a complete set of React hooks for working with
 * the Foundable Token Extended (VFT) contract in SailsJS-based apps.
 *
 * Includes examples:
 *  - Transaction hooks (mint, burn, transfer)
 *  - State queries (name, symbol, decimals, totalSupply, balanceOf)
 *  - Contract event subscriptions (minted, burned, transferred, approval)
 *
 * For full documentation, see the wiki:
 *   https://wiki.gear.foundation/docs/sails-js/react-hooks
 * For the VFT contract standard and API reference, see:
 *   https://wiki.gear.foundation/docs/examples/Standards/vft
 * ---------------------------------------------------------
 */

import { useProgram, useSendProgramTransaction, useProgramQuery, useProgramEvent } from '@gear-js/react-hooks';

import { ENV } from '../consts';

import { SailsProgram } from './lib';
import { TokenMetaQueries, TokenEventCallbacks } from './types.ts';

/**
 * Returns a contract program instance.
 *
 * This hook sets up the connection to your contract using its ABI (library) and on-chain id.
 * The returned instance is passed to all other hooks in this file.
 */
export function useProgramInstance() {
  return useProgram({
    library: SailsProgram, // ABI of the contract (must match what is deployed)
    id: ENV.CONTRACT, // Address (id) of the contract on-chain
  });
}

export function useSendMintTransaction() {
  const { data: program } = useProgramInstance();
  return useSendProgramTransaction({
    program,
    serviceName: 'vft',
    functionName: 'mint',
  });
}

export function useSendBurnTransaction() {
  const { data: program } = useProgramInstance();
  return useSendProgramTransaction({
    program,
    serviceName: 'vft',
    functionName: 'burn',
  });
}

export function useSendTransferTransaction() {
  const { data: program } = useProgramInstance();
  return useSendProgramTransaction({
    program,
    serviceName: 'vft',
    functionName: 'transfer',
  });
}

/**
 * Returns basic token metadata and utility states.
 *
 */
export function useTokenQueries(): TokenMetaQueries {
  const { data: program } = useProgramInstance();

  const { data: name, isPending: isNamePending } = useProgramQuery({
    program,
    serviceName: 'vft',
    functionName: 'name',
    args: [],
  });
  const { data: symbol, isPending: isSymbolPending } = useProgramQuery({
    program,
    serviceName: 'vft',
    functionName: 'symbol',
    args: [],
  });
  const { data: decimals, isPending: isDecimalsPending } = useProgramQuery({
    program,
    serviceName: 'vft',
    functionName: 'decimals',
    args: [],
  });
  const {
    data: totalSupply,
    isPending: isSupplyPending,
    refetch: refetchTotalSupply,
  } = useProgramQuery({
    program,
    serviceName: 'vft',
    functionName: 'totalSupply',
    args: [],
  });

  return {
    name,
    symbol,
    decimals,
    totalSupply,
    isLoading: isNamePending || isSymbolPending || isDecimalsPending || isSupplyPending,
    refetchTotalSupply,
  };
}

/**
 * Returns the balance for a given address.
 *
 * @param address ActorId (address, 0x-prefixed hex string)
 * @returns { balanceOf: any, ... }
 *
 * Note: This hook is parameterized, so you can call it with any address to check balance.
 * - The address param must always be present, and must be correctly encoded.
 * - `watch` can be enabled for live balance tracking, but default is false (manual query).
 */
export function useBalanceOfQuery(address: `0x${string}`) {
  const { data: program } = useProgramInstance();
  return useProgramQuery({
    program,
    serviceName: 'vft',
    functionName: 'balanceOf',
    args: [address],
    watch: false,
  });
}

/**
 * Subscribes to all relevant token contract events.
 *
 */
export function useTokenEvents(callbacks: TokenEventCallbacks) {
  const { data: program } = useProgramInstance();

  useProgramEvent({
    program,
    serviceName: 'vft',
    functionName: 'subscribeToMintedEvent',
    onData: (data) => {
      callbacks.onMinted?.(data);
    },
  });

  useProgramEvent({
    program,
    serviceName: 'vft',
    functionName: 'subscribeToBurnedEvent',
    onData: (data) => {
      callbacks.onBurned?.(data);
    },
  });

  useProgramEvent({
    program,
    serviceName: 'vft',
    functionName: 'subscribeToApprovalEvent',
    onData: (data) => {
      callbacks.onApproval?.(data);
    },
  });

  useProgramEvent({
    program,
    serviceName: 'vft',
    functionName: 'subscribeToTransferEvent',
    onData: (data) => {
      callbacks.onTransfer?.(data);
    },
  });
}
