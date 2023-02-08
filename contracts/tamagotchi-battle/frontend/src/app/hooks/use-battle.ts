import { useApp, useBattle } from 'app/context';
import { TamagotchiState } from '../types/lessons';
import { useEffect } from 'react';
import { BattleStateResponse } from '../types/battles';
import { HexString } from '@polkadot/util/types';
import { useAccount, useReadFullState, useSendMessage } from '@gear-js/react-hooks';
import { useMetadata } from './use-metadata';
import metaBattle from 'assets/meta/meta-battle.txt';
import metaPlayer from 'assets/meta/meta-tmg.txt';
import { ENV } from '../consts';

function useReadPlayerState<T>(player?: HexString) {
  const { metadata } = useMetadata(metaPlayer);
  return useReadFullState<T>(player, metadata);
}

function useReadBattleState<T>() {
  const { metadata } = useMetadata(metaBattle);
  return useReadFullState<T>(ENV.battle, metadata);
}

export function useInitBattleData() {
  const { setIsAdmin } = useApp();
  const { account } = useAccount();
  const { setPlayers, setBattleState } = useBattle();
  const { state } = useReadBattleState<BattleStateResponse>();
  // const { state: p1 } = useReadPlayerState<TamagotchiState>(state?.players[0]?.tmgId);
  // const { state: p2 } = useReadPlayerState<TamagotchiState>(state?.players[1]?.tmgId);

  // useEffect(() => {
  //   if (p1 && p2 && state) {
  //     setPlayers([
  //       { ...p1, ...state.players[0] },
  //       { ...p2, ...state.players[1] },
  //     ]);
  //   } else {
  //     setPlayers([]);
  //   }
  // }, [p1, p2, state]);

  useEffect(() => {
    setBattleState(state);
    if (state && account) {
      setIsAdmin(state.admin === account.decodedAddress);

      console.log({ state, players: Object.values(state.players) });
    }
  }, [state, account]);
}

export function useBattleMessage() {
  const { metadata } = useMetadata(metaBattle);
  return useSendMessage(ENV.battle, metadata);
}
