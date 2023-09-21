import { useEffect } from 'react'
import { useNavigate } from 'react-router-dom'
import {
  useAccount,
  useSendMessage,
} from '@gear-js/react-hooks'

import { useProgramMetadata } from '@/app/hooks/use-metadata'
import meta from '@/assets/meta/vara_man.meta.txt'
import { useGame } from '@/app/context/ctx-game'
import { useApp } from '../context/ctx-app'
import { programIdGame, useGameState } from './use-game-state'

export const useInitGame = () => {
  const { account } = useAccount()
  const { setIsSettled } = useApp()
  const { game, config, players, admins, error, status } = useGameState()
  const navigate = useNavigate();

  const { setGame, setIsAdmin, setPlayer, setAllPlayers, setConfigState, setStatus } = useGame()

  useEffect(() => {
    setConfigState(config?.Config || null)
    setIsSettled(!!config)
  }, [config?.Config])

  useEffect(() => {
    if (!programIdGame || !account?.decodedAddress) return

    if (game?.Game) {
      const gameCurrent = game.Game
      setGame(gameCurrent)
    } else {
      setGame(null)
    }
  }, [account?.decodedAddress, game?.Game])

  useEffect(() => {
    if (!programIdGame || !account?.decodedAddress) return

    if (players?.AllPlayers) {
      const playerCurrent = players.AllPlayers.find(x => x[0] === account.decodedAddress)
      setAllPlayers(players?.AllPlayers)

      if (playerCurrent && status?.Status !== "Paused") {
        setPlayer(playerCurrent[1])
        navigate('/levels');
      } else {
        navigate("/")
      }
    }
  }, [account?.decodedAddress, players?.AllPlayers])

  useEffect(() => {
    if (!programIdGame || !account?.decodedAddress) return

    if (admins?.Admins) {
      const isAdmin = admins.Admins.find((address) => address === account.decodedAddress)
      setIsAdmin(!!isAdmin)
    }
  }, [account?.decodedAddress, admins?.Admins])


  useEffect(() => {
    if (!programIdGame) return

    if (status?.Status) {
      setStatus(status?.Status)
    }
  }, [status?.Status])

  return {
    isGameReady: programIdGame ? Boolean(game) : true,
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
