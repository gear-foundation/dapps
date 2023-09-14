import { useNavigate } from 'react-router-dom'
import {
  useAccount,
  useReadFullState,
  useSendMessage,
} from '@gear-js/react-hooks'
import { ENV } from '@/app/consts'
import { useProgramMetadata, useReadState } from '@/app/hooks/use-metadata'
import metaTxt from '@/assets/meta/vara_man.meta.txt'
import { useApp } from '@/app/context/ctx-app'
import { useGame } from '@/app/context/ctx-game'
import { IGameState } from '@/app/types/game'
import { useEffect, useMemo } from 'react'

const programIdGame = ENV.GAME


export function useInitGame() {
  const navigate = useNavigate();
  const { setIsSettled } = useApp()
  const { account } = useAccount()
  const { game, setGame, setIsAdmin, setPlayer, setGamePlayer } = useGame()

  const payloadConfig = useMemo(() => ({ All: null }), [])

  const { state: config } = useReadState<{ All: IGameState }>({
    programId: programIdGame,
    meta: metaTxt,
    payload: payloadConfig,
  })

  useEffect(() => {
    const findPlayer = game?.players.find(([address]) => address === account?.decodedAddress)
    const isAdmin = game?.admins.find((address) => address === account?.decodedAddress)
    const findGamePlayer = findPlayer && game?.games.find(x => x[0] === findPlayer[0])

    setIsSettled(!!game)
    setIsAdmin(!!isAdmin)
    setPlayer(
      findPlayer
    )
    setGamePlayer(
      findGamePlayer
    )
    if (!findPlayer) {
      navigate('/');
    }
  }, [account, game])

  useEffect(() => {
    console.log('hello', config)
    if (!config) return

    setGame(config.All)
  }, [config, setGame])
}

export function useGameMessage() {
  const metadata = useProgramMetadata(metaTxt)
  return useSendMessage(programIdGame, metadata, {
    disableAlerts: true,
    isMaxGasLimit: true,
  })
}
