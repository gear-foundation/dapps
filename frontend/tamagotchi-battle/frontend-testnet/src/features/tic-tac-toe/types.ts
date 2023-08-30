import { HexString } from '@polkadot/util/types'

export interface IGameState {
  admin: string
  config: {
    ftContract: null | HexString
    nftMembershipGuard: null | HexString
    leaderboardContract: null | HexString
    tokensOnDraw: string
    tokensOnLose: string
    tokensOnWin: string
  }
  currentGames: Record<HexString, IGameInstance>
  leaderboard: Record<HexString, IPlayerStats>
}

export interface IPlayerStats {
  name: string
  points: string
  totalGames: string
  totalWins: string
}

export type ILeaderboardPlayer = IPlayerStats & {
  position: number
  address: HexString
}

export interface IGameInstance {
  board: Cell[]
  botMark: Mark
  playerMark: Mark
  lastTime: string
  gameOver: boolean
  gameResult: null | IGameResultStatus
}

export type IGameResultStatus = 'Player' | 'Bot' | 'Draw'

export enum Mark {
  X = 'X',
  O = 'O',
}

export type Cell = Mark | null
