import { HexString } from '@polkadot/util/types'
import { TamagotchiState } from './lessons'

export type BattlePlayerResponse = {
  attributes: string[]
  energy: string
  owner: HexString
  power: string
  tmgId: HexString
}

export type TamagotchiBattlePlayer = TamagotchiState & {
  attributes: string[]
  energy: string
  owner: HexString
  power: string
  tmgId: HexString
}

export type BattleStatesList =
  | 'Registration'
  | 'Moves'
  | 'Waiting'
  | 'GameIsOver'

export type BattleStateResponse = {
  currentTurn: string
  players: BattlePlayerResponse[]
  state: BattleStatesList
  steps: string
  tmgStoreId: HexString
  winner: HexString
}
