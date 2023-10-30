import { useAccount, useApi, useSendMessage } from '@gear-js/react-hooks'
import { useEffect, useMemo } from 'react'
import { useAtom, useAtomValue, useSetAtom } from 'jotai'
import isEqual from 'lodash.isequal'
import meta from './assets/meta/tic_tac_toe.meta.txt'
import {
  IDecodedReplyGame,
  IGameInstance,
  IQueryResponseConfig,
  IQueryResponseGame,
} from './types'
import { configAtom, countdownAtom, gameAtom, pendingAtom } from './store'
import { ADDRESS } from './consts'
import { useProgramMetadata } from '@/app/hooks'
import { useOnceReadState } from '@/app/hooks/use-once-read-state'
import { useWatchMessages } from '@/app/hooks/use-watch-messages'
import { toNumber } from '@/app/utils'

const programIdGame = ADDRESS.GAME

export function useGame() {
  const setGameState = useSetAtom(gameAtom)
  const gameState = useAtomValue(gameAtom)
  const setConfigState = useSetAtom(configAtom)
  const configState = useAtomValue(configAtom)
  const setCountdown = useSetAtom(countdownAtom)
  const countdown = useAtomValue(countdownAtom)

  const updateCountdown = (game: IGameInstance) => {
    setCountdown((prev) => {
      const timeLeft =
        toNumber(game.lastTime) + toNumber(configState?.turnDeadlineMs || '0')
      const isPassed = Date.now() - timeLeft > 0
      const isNew = prev?.value !== game.lastTime

      console.log('Last Time:')
      console.log(game.lastTime)
      console.log('turnDeadlineMs:')
      console.log(configState?.turnDeadlineMs)
      console.log('timeLeft:')
      console.log(timeLeft)
      console.log('isPassed:')
      console.log(isPassed)
      console.log('isNew:')
      console.log(isNew)

      return isNew
        ? { value: game.lastTime, isActive: isNew && !isPassed }
        : prev
    })
  }

  const updateGame = (game: IGameInstance) => {
    console.log('Game recieved:')
    console.log(game)
    setGameState(game)
    updateCountdown(game)
  }

  const resetGame = () => {
    console.log('reset all')
    setGameState(null)
    setCountdown(undefined)
  }

  return {
    resetGame,
    setGameState,
    gameState,
    setCountdown,
    countdown,
    setConfigState,
    configState,
    updateCountdown,
    updateGame,
  }
}

export function useOnceGameState() {
  const { account } = useAccount()

  const payloadGame = useMemo(
    () =>
      account?.decodedAddress
        ? { Game: { player_id: account.decodedAddress } }
        : undefined,
    [account?.decodedAddress]
  )
  const payloadConfig = useMemo(() => ({ Config: null }), [])

  console.log(account?.decodedAddress)
  console.log(!!meta)

  const {
    state: stateConfig,
    error: configError,
    handleReadState: triggerConfig,
  } = useOnceReadState<IQueryResponseConfig>({
    programId: programIdGame,
    payload: payloadConfig,
    meta,
  })

  const {
    state: stateGame,
    error: gameError,
    handleReadState: triggerGame,
  } = useOnceReadState<IQueryResponseGame>({
    programId: programIdGame,
    payload: payloadGame,
    meta,
  })

  return {
    stateGame,
    stateConfig,
    error: gameError || configError,
    triggerGame,
    triggerConfig,
  }
}

export const useInitGame = () => {
  const { account } = useAccount()
  const { gameState } = useGame()

  console.log(gameState)

  return {
    isGameReady: account?.decodedAddress ? gameState !== undefined : true,
  }
}
export const useInitGameSync = () => {
  const { isApiReady, api } = useApi()
  const { account } = useAccount()
  const { stateGame, stateConfig, error, triggerGame, triggerConfig } =
    useOnceGameState()
  const { updateGame, resetGame, setConfigState } = useGame()

  useEffect(() => {
    console.log('FETCH CONFIG:')
    console.log(!!isApiReady)
    console.log(!!api)
    console.log(!!account?.decodedAddress)
    console.log(!!meta)

    if (!isApiReady || !api || !meta) return

    console.log('fetch config', isApiReady)
    triggerConfig()

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isApiReady, api, meta])

  useEffect(() => {
    if (!isApiReady || !api || !meta || !stateConfig?.Config) return

    console.log('trigger game state')
    triggerGame()
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isApiReady, api, meta, stateConfig?.Config, account?.decodedAddress])

  useEffect(() => {
    if (!stateConfig?.Config) return

    setConfigState(stateConfig.Config)
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [stateConfig?.Config])

  useEffect(() => {
    console.log(stateGame)
    console.log(stateConfig)
    if (stateGame === undefined || !stateConfig?.Config) return

    console.log({ stateGame })

    const game = stateGame?.Game

    console.log('GAME')
    console.log(game)
    game ? updateGame(game) : resetGame()

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [stateGame?.Game, stateConfig?.Config])

  return {
    errorGame: error,
  }
}

export function useGameMessage() {
  const metadata = useProgramMetadata(meta)
  return useSendMessage(programIdGame, metadata, {
    disableAlerts: true,
    isMaxGasLimit: true,
  })
}

export function usePending() {
  const [pending, setPending] = useAtom(pendingAtom)
  return { pending, setPending }
}

export function useSubscriptionOnGameMessage() {
  const { gameState, updateGame } = useGame()
  const { subscribe, unsubscribe, reply, isOpened } =
    useWatchMessages<IDecodedReplyGame>()

  useEffect(() => {
    if (!isOpened) return
    console.log('received: ', reply)
    const game = reply?.MoveMade?.game || reply?.GameStarted?.game

    if (game && !isEqual(game.board, gameState?.board)) {
      updateGame(game)
      unsubscribe()
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [reply, isOpened])

  return {
    subscribe,
    unsubscribe,
    reply,
    isOpened,
  }
}
