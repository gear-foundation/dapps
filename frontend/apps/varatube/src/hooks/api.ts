import { useAtom } from 'jotai';
import { useAccount, useApi, useReadFullState, useSendMessage, useSendMessageWithGas } from '@gear-js/react-hooks';
import { HexString } from '@polkadot/util/types';
import varatubeMeta from 'assets/state/varatube_meta.txt';
import ftMeta from 'assets/state/ft_meta.txt';
import { ADDRESS } from 'consts';
import { useProgramMetadata } from './metadata';
import { useCallback, useEffect } from 'react';
import { IS_STATE_READ_ATOM, STATE_ATOM } from 'atoms';
import { FullSubState } from 'types';

function useSubscriptionMeta() {
  return useProgramMetadata(varatubeMeta);
}

function useFTMeta() {
  return useProgramMetadata(ftMeta);
}

function useSubscriptionsMessage() {
  const metadata = useSubscriptionMeta();

  return useSendMessage(ADDRESS.CONTRACT, metadata);
}

function useFTMessage() {
  const metadata = useFTMeta();

  return useSendMessageWithGas(ADDRESS.FT_CONTRACT, metadata, { isMaxGasLimit: true });
}

type FTState = { balances: [[HexString, string]] };

function useFTBalance() {
  const { account } = useAccount();
  const { decodedAddress } = account || {};

  const meta = useProgramMetadata(ftMeta);
  const { state } = useReadFullState<FTState>(ADDRESS.FT_CONTRACT, meta, '0x');

  const balances = state?.balances;
  const userBalanceEntity = balances?.find(([address]) => address === decodedAddress);
  const [, balance] = userBalanceEntity || [];

  return balance;
}

function useProgramState() {
  const meta = useSubscriptionMeta();
  const { api } = useApi();
  const programId = ADDRESS.CONTRACT;
  const [data, setData] = useAtom(STATE_ATOM);
  const [isStateRead, setIsStateRead] = useAtom(IS_STATE_READ_ATOM);

  const triggerState = useCallback(() => {
    if (!api || !meta || !programId) return;

    const payload = {
      Subscribers: null,
    };

    api.programState
      .read({ programId, payload }, meta)
      .then((codec) => codec.toHuman())
      .then((state: any) => {
        setData(state);
        setIsStateRead(true);
      });
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [api, meta, programId, setData]);

  useEffect(() => {
    if (!isStateRead) {
      triggerState();
    }
  }, [isStateRead, triggerState]);

  const state = {
    subscriptionsState: data?.Subscribers || null,
    isSubscriptionsStateRead: isStateRead,
    updateState: triggerState,
  };

  return state;
}

export { useSubscriptionsMessage, useFTBalance, useFTMessage, useProgramState };
