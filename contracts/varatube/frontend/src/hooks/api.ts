import { getProgramMetadata } from '@gear-js/api';
import { useAccount, useReadFullState, useReadWasmState, useSendMessage } from '@gear-js/react-hooks';
import { HexString } from '@polkadot/util/types';
import { useState, useEffect } from 'react';
import stateWasm from 'assets/gear_subscription_state.meta.wasm';
import { ADDRESS, META_HEX } from 'consts';

const metadata = getProgramMetadata(META_HEX.SUBSCRIPTION);

type FullSubState = {
  [key: HexString]: {
    isActive: boolean;
    startDate: number;
    endDate: number;
    startBlock: number;
    endBlock: number;
    period: string;
    renewalDate: number;
    renewalBlock: number;
    price: number;
    willRenew: boolean;
  };
};

function useSubscriptions() {
  const [buffer, setBuffer] = useState<Buffer>();

  useEffect(() => {
    fetch(stateWasm)
      .then((result) => result.arrayBuffer())
      .then((arrBuffer) => Buffer.from(arrBuffer))
      .then((res) => setBuffer(res));
  }, []);

  const { state, isStateRead } = useReadWasmState<FullSubState>(ADDRESS.CONTRACT, buffer, 'all_subscriptions', null);

  return { subscriptionsState: state, isSubscriptionsStateRead: isStateRead };
}

function useSubscriptionsMessage() {
  return useSendMessage(ADDRESS.CONTRACT, metadata, true);
}

type FTState = { balances: [[HexString, number]] };

function useFTBalance() {
  const { account } = useAccount();
  const { decodedAddress } = account || {};

  const meta = getProgramMetadata(META_HEX.FT);
  const { state, isStateRead } = useReadFullState<FTState>(ADDRESS.FT_CONTRACT, meta);

  const balances = state?.balances;
  const userBalanceEntity = balances?.find(([address]) => address === decodedAddress);
  const [, balance] = userBalanceEntity || [];

  return balance;
}

export { useSubscriptions, useSubscriptionsMessage, useFTBalance };
