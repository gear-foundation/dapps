import { useApp, useBattle } from 'app/context';
import { useEffect } from 'react';
import type { BattlePlayerType, BattleStateResponse } from 'app/types/battles';
import { useAccount, useReadFullState } from '@gear-js/react-hooks';
import { useMetadata } from './use-metadata';
import meta from 'assets/meta/meta.txt';
import { ENV } from 'app/consts';
import { useSendMessage } from './useSendMessage';

function useReadGameState<T>() {
  const { metadata } = useMetadata(meta);
  return useReadFullState<T>(ENV.game, metadata);
}

export function useInitGame() {
  const { setIsAdmin } = useApp();
  const { account } = useAccount();
  const { setRivals, setBattle, setCurrentPlayer, setPlayers } = useBattle();
  const { state } = useReadGameState<BattleStateResponse>();

  useEffect(() => {
    setBattle(state);
    if (state && account) {
      // setIsAdmin(state.admin === account.decodedAddress);
      // const getPlayers = () => {
      // const result: BattlePlayerType[] = [];
      // state.round.tmgIds.forEach((player, i) => {
      //   if (state.players[player]) result.push(state.players[player]);
      // });
      // return result;
      // };
      // setPlayers(Object.values(state.players));
      // setRivals(getPlayers());
      // setCurrentPlayer(state.round.tmgIds[state.round.moves.length > 0 ? 1 : 0]);
    } else {
      // setPlayers([]);
      // setRivals([]);
    }
  }, [state, account]);
}

export function useGameMessage() {
  const { metadata } = useMetadata(meta);
  return useSendMessage(ENV.game, metadata);
}
