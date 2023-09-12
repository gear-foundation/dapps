import { useNavigate } from 'react-router-dom'
import {
  useAccount,
  useReadFullState,
  useSendMessage,
} from '@gear-js/react-hooks'
import { ENV } from '@/app/consts'
import { useProgramMetadata, useReadState } from '@/app/hooks/use-metadata'
import metaTxt from '@/assets/meta/vara_man_game.meta.txt'
import { useApp } from '@/app/context/ctx-app'
import { useGame } from '@/app/context/ctx-game'
import { IGameState } from '@/app/types/game'
import { useEffect } from 'react'

const programIdGame = ENV.GAME


export function useInitGame() {
  const navigate = useNavigate();
  const { setIsSettled } = useApp()
  const { account } = useAccount()
  const { game, setGame, setIsAdmin, setPlayer } = useGame()
  const { state, error } = useReadState<IGameState>({
    programId: programIdGame,
    meta: metaTxt,
  })

  useEffect(() => {
    const findPlayer = game?.players.find(([address]) => address === account?.decodedAddress)
    const isAdmin = game?.admins.find((address) => address === account?.decodedAddress)

    setIsSettled(!!game)
    setIsAdmin(!!isAdmin)
    setPlayer(
      findPlayer
    )

    if (!findPlayer) {
      navigate('/');
    }
  }, [account, game])

  useEffect(() => {
    console.log('hello', state)
    setGame(state)
  }, [state, setGame])
}

export function useGameMessage() {
  const metadata = useProgramMetadata(metaTxt)
  return useSendMessage(programIdGame, metadata, {
    disableAlerts: true,
    isMaxGasLimit: true,
  })
}
