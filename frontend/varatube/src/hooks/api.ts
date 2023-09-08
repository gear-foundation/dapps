import { useAccount, useReadFullState, useReadWasmState, useSendMessage } from '@gear-js/react-hooks';
import { HexString } from '@polkadot/util/types';
import stateWasm from 'assets/state/varatube_state.meta.wasm';
import varatubeMeta from 'assets/state/varatube_meta.txt';
import ftMeta from 'assets/state/ft_meta.txt';
import { ADDRESS } from 'consts';
import { useBuffer, useProgramMetadata } from './metadata';

type FullSubState = {
  [key: HexString]: {
    isActive: boolean;
    startDate: string;
    endDate: string;
    startBlock: string;
    endBlock: string;
    period: string;
    renewalDate: string;
    renewalBlock: string;
    price: string;
    willRenew: boolean;
  };
};

function useSubscriptions() {
  const buffer = useBuffer(stateWasm);
  const { state, isStateRead } = useReadWasmState<FullSubState>(ADDRESS.CONTRACT, buffer, 'all_subscriptions', null);

  return { subscriptionsState: state, isSubscriptionsStateRead: isStateRead };
}

function useSubscriptionsMessage() {
  const metadata = useProgramMetadata(varatubeMeta);

  return useSendMessage(ADDRESS.CONTRACT, metadata, { isMaxGasLimit: true });
}

type FTState = { balances: [[HexString, string]] };

function useFTBalance() {
  const { account } = useAccount();
  const { decodedAddress } = account || {};

  const meta = useProgramMetadata(ftMeta);
  const { state } = useReadFullState<FTState>(ADDRESS.FT_CONTRACT, meta);

  const balances = state?.balances;
  const userBalanceEntity = balances?.find(([address]) => address === decodedAddress);
  const [, balance] = userBalanceEntity || [];

  return balance;
}

export { useSubscriptions, useSubscriptionsMessage, useFTBalance };
