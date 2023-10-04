import { HexString } from '@polkadot/util/types'

export type TamagotchiColor =
  | 'Green'
  | 'Red'
  | 'Blue'
  | 'Purple'
  | 'Orange'
  | 'Yellow'
export type PlayerColor = TamagotchiColor
export type BattleStatus =
  | 'Unknown'
  | 'Registration'
  | 'GameIsOn'
  | 'GameIsOver'
export type BattleHeroStatuses = 'Unknown' | 'Registered' | 'Winner' | 'Played'
export type BattleRoundMoveVariants = 'Defence' | 'Attack' | 'Skipped'

export type RoundDamageType = [
  string,
  string,
  string,
  BattleRoundMoveVariants,
  BattleRoundMoveVariants
]

export type BattleStatePair = {
  gameIsOver: boolean
  moveDeadline: string
  moves: []
  ownerIds: HexString[]
  rounds: string
  tmgIds: HexString[]
  winner: HexString
}

export type BattleStatePlayer = {
  color: PlayerColor
  dateOfBirth: string
  defence: string
  health: string
  name: string
  owner: HexString
  power: string
  tmgId: HexString
  victories: string
}

export type BattleHeroMove = 'Attack' | 'Defence'

export type BattleHero = {
  status: BattleHeroStatuses
  room_num: number
  round: number
  coupled: HexString
  last_move: BattleHeroMove
  victories: number

  owner: HexString
  hero_id: HexString
  name: string
  dateOfBirth: string
  defense: string
  power: string
  health: string
  color: TamagotchiColor
}

export type BattleRoom = {
  status: BattleStatus
  delayed_msg: string
  round: number
}

export type BattleStateResponse = {
  admins: HexString[]
  currentReservation: null
  heroes: Record<HexString, BattleHero>
  rooms: BattleRoom[]
  status: BattleStatus
  tournamentsStartTimestamps: Record<string, HexString>
}
