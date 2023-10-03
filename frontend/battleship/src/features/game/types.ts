import { HexString } from '@polkadot/util/types'

export interface IGameState {
  botAddress: string
  games: Array<[HexString, IGameInstance]>
}

export interface IGameInstance {
  botBoard: string[]
  botShips: string[]
  playerBoard: string[]
  playerShips: string[]
  gameOver: boolean
  gameResult: null | IGameResultStatus
  turn: 'Player' | 'Bot'
  startTime: string
  endTime: string
  totalShots: string
}

export type IGameResultStatus = 'Player' | 'Bot' | 'Draw'
