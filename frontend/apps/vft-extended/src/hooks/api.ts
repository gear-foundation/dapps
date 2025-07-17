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
 *
 * For full documentation, see the wiki:
 *   https://wiki.gear.foundation/docs/sails-js/react-hooks
 *
 * For the VFT contract standard and API reference, see:
 *   https://wiki.gear.foundation/docs/examples/Standards/vft
 * ---------------------------------------------------------
 */

import { useProgram, useSendProgramTransaction, useProgramQuery, useProgramEvent } from '@gear-js/react-hooks';

import { ENV } from '../consts';

import { SailsProgram } from './lib'
import { TokenMetaQueries, TokenEventCallbacks } from './types.ts'

/**
 * Returns a contract program instance.
 * 
 * This hook sets up the connection to your contract using its ABI (library) and on-chain id.
 * The returned instance is passed to all other hooks in this file.
 * 
 * Note: You can use another id (address) or ABI to connect to different contracts.
 */
export function useProgramInstance() {
  return useProgram({
    library: SailsProgram,      // ABI of the contract (must match what is deployed)
    id: ENV.CONTRACT,           // Address (id) of the contract on-chain
  });
}

/**
 * Returns async token actions and their loading states.
 *
 * This hook prepares all common token operations: mint, burn, transfer.
 * Each action is async and can be awaited; you get the current loading (pending) state for each.
 * 
 * Note: Each action uses serviceName and functionName matching the contract API.
 * You can easily add more actions if needed (see sendTransactionAsync usage).
 */
export function useTokenActions() {
  const { data: program } = useProgramInstance();

  // Hook for minting tokens
  const {
    sendTransactionAsync: sendMintTransactionAsync,
    isPending: mintPending,
  } = useSendProgramTransaction({
    program,
    serviceName: 'vft',
    functionName: 'mint',
  });

  // Hook for burning tokens
  const {
    sendTransactionAsync: sendBurnTransactionAsync,
    isPending: burnPending,
  } = useSendProgramTransaction({
    program,
    serviceName: 'vft',
    functionName: 'burn',
  });

  // Hook for transferring tokens
  const {
    sendTransactionAsync: sendTransferTransactionAsync,
    isPending: transferPending,
  } = useSendProgramTransaction({
    program,
    serviceName: 'vft',
    functionName: 'transfer',
  });

  /**
   * Async wrapper for minting tokens.
   * @param to Recipient address (must be a hex string with 0x prefix)
   * @param value Amount to mint (string, because JS can't safely handle big numbers)
   */
  const mint = async (to: `0x${string}`, value: string) => {
    return sendMintTransactionAsync({
      args: [to, value],
    });
  };

  /**
   * Async wrapper for burning tokens.
   * @param from Address to burn from
   * @param value Amount to burn
   */
  const burn = async (from: `0x${string}`, value: string) => {
    return sendBurnTransactionAsync({
      args: [from, value],
    });
  };

  /**
   * Async wrapper for transferring tokens.
   * @param to Recipient address
   * @param value Amount to transfer
   */
  const transfer = async (to: `0x${string}`, value: string) => {
    return sendTransferTransactionAsync({ args: [to, value] });
  };

  // You can add more methods here, following the same pattern

  return { mint, mintPending, burn, burnPending, transfer, transferPending };
}

/**
 * Returns basic token metadata and utility states.
 * 
 * This hook uses useProgramQuery to read the name, symbol, decimals, and total supply from the contract.
 * 
 * Note: 
 * - Each query has its own loading state, here combined into isLoading for simplicity.
 * - The returned refetchTotalSupply lets you manually trigger an update if you want to listen to events.
 * - The underlying useProgramQuery accepts a `watch` option (default false): if true, will refetch on every block!
 */
export function useTokenQueries(): TokenMetaQueries {
  const { data: program } = useProgramInstance();

  // Query for token name
  const { data: name, isPending: isNamePending } = useProgramQuery({
    program,
    serviceName: 'vft',
    functionName: 'name',
    args: [],
    // watch: true // Enable if you want live updates each block (rarely needed for static data)
  });
  // Query for token symbol
  const { data: symbol, isPending: isSymbolPending } = useProgramQuery({
    program,
    serviceName: 'vft',
    functionName: 'symbol',
    args: [],
  });
  // Query for token decimals
  const { data: decimals, isPending: isDecimalsPending } = useProgramQuery({
    program,
    serviceName: 'vft',
    functionName: 'decimals',
    args: [],
  });
  // Query for token total supply
  const { data: totalSupply, isPending: isSupplyPending, refetch: refetchTotalSupply } = useProgramQuery({
    program,
    serviceName: 'vft',
    functionName: 'totalSupply',
    args: [],
  });

  return {
    name, symbol, decimals, totalSupply,
    isLoading: isNamePending || isSymbolPending || isDecimalsPending || isSupplyPending,
    refetchTotalSupply
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

  const balanceOf = useProgramQuery({
    program,
    serviceName: 'vft',
    functionName: 'balanceOf',
    args: [address],
    watch: false,
  });

  return balanceOf;
}

/**
 * Subscribes to all relevant token contract events.
 * 
 * This hook lets you react to on-chain events (e.g., minted, burned, transferred, approved) from the contract.
 * 
 * - Each event has its own callback.
 * - You can use these to trigger UI updates (like refetching supply or balances).
 * 
 * Note: You can subscribe to more events, depending on your contract ABI.
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

  // You can add more subscriptions for custom events if your contract supports them
}