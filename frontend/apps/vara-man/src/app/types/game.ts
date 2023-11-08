import { HexString } from '@polkadot/util/types'

export type IGameState = {
  admins: string[]
  games: IGame[]
  players: IPlayer[]
  config: IGameConfig
}

export type IGameStatus = 'Paused' | 'Started'

export type IGameConfig = {
  oneCoinInValue: string
  tokensPerGoldCoinEasy: string
  tokensPerSilverCoinEasy: string
  tokensPerGoldCoinMedium: string
  tokensPerSilverCoinMedium: string
  tokensPerGoldCoinHard: string
  tokensPerSilverCoinHard: string
  goldCoins: string
  silverCoins: string
  numberOfLives: string
}

export type IPlayer = [HexString, IPlayerInfo]
export type IGame = [string, IGameInstance]

export type IPlayerInfo = {
  name: string // Имя
  lives: string // Количество попыток(игр)
  claimedGoldCoins: number // Количество заработанных золотых монет
  claimedSilverCoins: number // Количество заработанных серебряных монет
}

export type IGameLevel = 'Easy' | 'Medium' | 'Hard'

export interface IGameLevelConfig {
  speed: number // Скорость
  map: number[][] // Карта уровня
}

export type IGameInstance = {
  level: IGameLevel // Уровень сложности
  playerAddress: HexString // Адрес игрока
  gold_coins: number // Количество золотых монет на карте
  silver_coins: number // Количество серебряных монет на карте
  start_time_ms: number // Время начала игры
  isClaimed: boolean // Флаг, который указывает на то, забрал ли игрок награду(клейм) или нет
}
