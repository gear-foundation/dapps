import { useReadState } from '@/app/hooks/api';
import { useMemo } from 'react';
import {
  BattleAdminsResponse,
  BattleCompletedGamesResponse,
  BattleCurrentPlayersResponse,
  BattleCurrentWinnerResponse,
  BattleGameStateResponse,
  BattlePairsResponse,
  BattlePlayersIdsResponse,
  BattlePlayersResponse,
} from '../types/battles';
import { HexString } from '@gear-js/api';

export const useFullState = (programId: HexString, meta: string) => {
  const getPairsPayload = useMemo(() => ({ GetPairs: null }), []);
  const adminsPayload = useMemo(() => ({ Admins: null }), []);
  const playersPayload = useMemo(() => ({ Players: null }), []);
  const playerIdsPayload = useMemo(() => ({ PlayersIds: null }), []);
  const currentPlayersPayload = useMemo(() => ({ CurrentPlayers: null }), []);
  const completedGamesPayload = useMemo(() => ({ CompletedGames: null }), []);
  const winnerPayload = useMemo(() => ({ Winner: null }), []);
  const statePayload = useMemo(() => ({ State: null }), []);

  const { state: pairsState } = useReadState<BattlePairsResponse>({ programId, meta, payload: getPairsPayload });
  const { state: adminsState } = useReadState<BattleAdminsResponse>({ programId, meta, payload: adminsPayload });
  const { state: playersState } = useReadState<BattlePlayersResponse>({ programId, meta, payload: playersPayload });
  const { state: playersIdsState } = useReadState<BattlePlayersIdsResponse>({
    programId,
    meta,
    payload: playerIdsPayload,
  });
  const { state: currentPlayersState } = useReadState<BattleCurrentPlayersResponse>({
    programId,
    meta,
    payload: currentPlayersPayload,
  });
  const { state: completedGamesState } = useReadState<BattleCompletedGamesResponse>({
    programId,
    meta,
    payload: completedGamesPayload,
  });
  const { state: currentWinnerState } = useReadState<BattleCurrentWinnerResponse>({
    programId,
    meta,
    payload: winnerPayload,
  });
  const { state: battleState } = useReadState<BattleGameStateResponse>({ programId, meta, payload: statePayload });

  const state = useMemo(
    () =>
      !!pairsState &&
      !!adminsState &&
      !!currentPlayersState &&
      !!playersState &&
      !!completedGamesState &&
      !!currentWinnerState &&
      !!battleState &&
      !!playersIdsState
        ? {
            admins: adminsState.Admins.admins,
            completedGames: completedGamesState.CompletedGames.completedGames,
            currentWinner: currentWinnerState.Winner.winner,
            players: playersState.Players.players,
            playersIds: playersIdsState.PlayersIds.playersIds,
            currentPlayers: currentPlayersState.CurrentPlayers.currentPlayers,
            state: battleState.State.state,
            pairs: pairsState.Pairs.pairs,
          }
        : undefined,
    [
      pairsState,
      adminsState,
      currentPlayersState,
      playersState,
      completedGamesState,
      currentWinnerState,
      battleState,
      playersIdsState,
    ],
  );

  return state;
};
