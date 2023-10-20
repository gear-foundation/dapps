import { TamagotchiState } from '../types/lessons'
import { useEffect } from 'react'
import { useSendMessage } from '@gear-js/react-hooks'
import { useProgramMetadata, useReadState } from './use-metadata'
import metaBattle from '@/assets/meta/meta-battle.txt'
import metaPlayer from '@/assets/meta/meta6.txt'
import { ENV } from '@/app/consts'
import { useBattle } from '@/app/context'
import type { BattleStateResponse } from '@/app/types/battles'

export function useInitBattleData() {
  const { setPlayers, setBattleState } = useBattle()

  const state = useReadState<BattleStateResponse>({
    programId: ENV.battle,
    meta: metaBattle,
  }).state
  const p1 = useReadState<TamagotchiState>({
    programId: state?.players[0]?.tmgId,
    meta: metaPlayer,
  }).state
  const p2 = useReadState<TamagotchiState>({
    programId: state?.players[1]?.tmgId,
    meta: metaPlayer,
  }).state

  useEffect(() => {
    if (p1 && p2 && state) {
      setPlayers([
        { ...p1, ...state.players[0] },
        { ...p2, ...state.players[1] },
      ])
    } else {
      setPlayers([])
    }
  }, [p1, p2, state])

  useEffect(() => {
    setBattleState(state)
  }, [state])
}

export function useBattleMessage() {
  const metadata = useProgramMetadata(metaBattle)
  return useSendMessage(ENV.battle, metadata)
}
