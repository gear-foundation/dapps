import { useAccount, useSendMessage } from '@gear-js/react-hooks'
import { useEffect, useMemo } from 'react'
import { useAtom, useAtomValue, useSetAtom } from 'jotai'
import metaTxt from './assets/meta/tic_tac_toe.meta.txt'
import { IGameState, ILeaderboardPlayer, IPlayerStats } from './types'
import {
  configAtom,
  countdownAtom,
  gameAtom,
  leaderboardAtom,
  pendingAtom,
} from './store'
import { ADDRESS } from './consts'
import { useProgramMetadata } from '@/app/hooks'
import { useReadState } from '@/app/hooks/api'
import { HexString } from '@polkadot/util/types'
import { toNumber } from '@/app/utils'

const programIdGame = ADDRESS.GAME

export function useGame() {
  const setGameState = useSetAtom(gameAtom)
  const gameState = useAtomValue(gameAtom)
  const setCountdown = useSetAtom(countdownAtom)
  const countdown = useAtomValue(countdownAtom)
  const setGameConfig = useSetAtom(configAtom)
  const gameConfig = useAtomValue(configAtom)
  const setLeaderboard = useSetAtom(leaderboardAtom)
  const leaderboard = useAtomValue(leaderboardAtom)

  const resetGameState = () => {
    setGameState(undefined)
    setCountdown(undefined)
    setGameConfig(undefined)
  }

  return {
    resetGameState,
    setGameState,
    gameState,
    setCountdown,
    countdown,
    setGameConfig,
    gameConfig,
    setLeaderboard,
    leaderboard,
  }
}

export const useInitGame = () => {
  const { account } = useAccount()
  const { state, error } = useReadState<IGameState>({
    programId: programIdGame,
    meta: metaTxt,
  })
  const {
    setGameState,
    setLeaderboard,
    setGameConfig,
    setCountdown,
    resetGameState,
  } = useGame()

  const stats = useMemo<ILeaderboardPlayer[]>(() => {
    const s = state?.leaderboard
    if (!!s) {
      const entries = Object.entries(s) as [HexString, IPlayerStats][]
      const playersSorted = entries.sort(([, p1], [, p2]) =>
        toNumber(p1.points) > toNumber(p2.points) ? -1 : 1
      )
      return playersSorted.map(([address, stats], i) => {
        return {
          position: i + 1,
          address,
          ...stats,
        }
      })
    }
    return []
  }, [state?.leaderboard])

  useEffect(() => {
    setLeaderboard(stats)
  }, [stats])

  useEffect(() => {
    if (programIdGame && account) {
      // console.log({ state })

      if (state && !!Object.keys(state?.currentGames || {}).length) {
        const game = state.currentGames[account.decodedAddress]

        if (game) {
          setGameState(game)
          setGameConfig(state.config)
          setCountdown((prev) => {
            const isNew = prev?.value !== game.lastTime
            return isNew ? { value: game.lastTime, isActive: isNew } : prev
          })
        } else {
          resetGameState()
        }
      }
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [account, state])

  return {
    isGameReady: programIdGame ? Boolean(state) : true,
    errorGame: error,
  }
}

export function useGameMessage() {
  const metadata = useProgramMetadata(metaTxt)
  return useSendMessage(programIdGame, metadata, {
    disableAlerts: true,
    isMaxGasLimit: true,
  })
}

export function usePending() {
  const [pending, setPending] = useAtom(pendingAtom)
  return { pending, setPending }
}
