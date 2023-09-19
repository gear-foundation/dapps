import { useMemo } from "react"
import { useAccount } from "@gear-js/react-hooks"
import { ENV } from '@/app/consts'
import { useReadState } from "./use-metadata"
import meta from '@/assets/meta/vara_man.meta.txt'
import { IGameConfig, IGameInstance, IGameStatus, IPlayer } from '@/app/types/game'

export const programIdGame = ENV.GAME

export function useGameState() {
    const { account } = useAccount()

    const payloadGame = useMemo(
        () =>
            account?.decodedAddress
                ? {
                    Game: { player_address: account.decodedAddress },
                }
                : undefined,
        [account?.decodedAddress])

    const payloadConfig = useMemo(() => ({ Config: null }), [])
    const payloadAdmins = useMemo(() => ({ Admins: null }), [])
    const payloadPlayers = useMemo(() => ({ AllPlayers: null }), [])
    const payloadStatus = useMemo(() => ({ Status: null }), [])

    const { state: game, error } = useReadState<{ Game: IGameInstance }>({
        programId: programIdGame,
        meta,
        payload: payloadGame,
    })

    const { state: config } = useReadState<{ Config: IGameConfig | null }>({
        programId: programIdGame,
        meta,
        payload: payloadConfig,
    })

    const { state: players } = useReadState<{ AllPlayers: IPlayer[] }>({
        programId: programIdGame,
        meta,
        payload: payloadPlayers,
    })

    const { state: admins } = useReadState<{ Admins: string[] }>({
        programId: programIdGame,
        meta,
        payload: payloadAdmins,
    })

    const { state: status } = useReadState<{ Status: IGameStatus }>({
        programId: programIdGame,
        meta,
        payload: payloadStatus,
    })

    return { game, config, players, admins, error, status }
}