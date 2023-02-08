import { useApp, useBattle } from 'app/context';
import { useEffect } from 'react';
import { BattlePlayerType, BattleStateResponse } from '../types/battles';
import { useAccount, useReadFullState, useSendMessage } from '@gear-js/react-hooks';
import { useMetadata } from './use-metadata';
import metaBattle from 'assets/meta/meta-battle.txt';
import { ENV } from '../consts';

function useReadBattleState<T>() {
  const { metadata } = useMetadata(metaBattle);
  return useReadFullState<T>(ENV.battle, metadata);
}

export function useInitBattleData() {
  const { setIsAdmin } = useApp();
  const { account } = useAccount();
  const { setPlayers, setBattleState } = useBattle();
  const { state } = useReadBattleState<BattleStateResponse>();

  useEffect(() => {
    setBattleState(state);
    if (state && account) {
      setIsAdmin(state.admin === account.decodedAddress);

      const getPlayers = () => {
        const result: BattlePlayerType[] = [];
        state.round.tmgIds.forEach((player, i) => {
          if (state.players[player]) result.push(state.players[player]);
        });
        return result;
      };

      setPlayers(getPlayers());

      console.log({ state, players: Object.values(state.players) });
    } else {
      setPlayers([]);
    }
  }, [state, account]);
}

export function useBattleMessage() {
  const { metadata } = useMetadata(metaBattle);
  return useSendMessage(ENV.battle, metadata);
}
