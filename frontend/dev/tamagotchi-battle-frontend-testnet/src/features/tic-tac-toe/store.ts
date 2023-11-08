import { atom } from 'jotai'
import { IGameInstance, IGameState, ILeaderboardPlayer } from './types'

export const gameAtom = atom<IGameInstance | undefined>(undefined)
export const configAtom = atom<IGameState['config'] | undefined>(undefined)
export const leaderboardAtom = atom<ILeaderboardPlayer[]>([])

export const pendingAtom = atom<boolean>(false)
export const countdownAtom = atom<
  { isActive: boolean; value: string } | undefined
>(undefined)
