import { HexString } from '@polkadot/util/types'

export type IGameState = {
	admins: string[]
	games: IGame[]
	players: IPlayer[]
	config: IGameConfig
}

export type IGameStatus = 'Paused' | 'Started'

export type IPlayer = [HexString, IPlayerInfo]
export type IGame = [string, IGameInstance]

export type IPlayerInfo = {
	name: string
	lives: string
	claimedGoldCoins: number
	claimedSilverCoins: number
}

export interface IGameLevelConfig {
	speed: number
	map: number[][]
}

export type IGameLevel = 'Easy' | 'Medium' | 'Hard'

export type IGameConfig = {
	gasForFinishSingleGame: string
	gasForFinishTournament: string
	onePointInValue: string
	pointsPerGoldCoinEasy: string
	pointsPerGoldCoinHard: string
	pointsPerGoldCoinMedium: string
	pointsPerSilverCoinEasy: string
	pointsPerSilverCoinHard: string
	pointsPerSilverCoinMedium: string
	timeForSingleRound: string
}

interface SingleGameState {
	type: 'SingleGame'
	SingleGame: ISingleGameInstance
}

interface TournamentState {
	type: 'Tournament'
	TournamentGame: ITournamentGameInstance
}

export type GameState = SingleGameState | TournamentState

export type ISingleGameInstance = [
	{
		level: 'Easy' | 'Medium' | 'Hard'
		points: string
		startTime: string
		gameOver: boolean
	},
	string
]

export type ITournamentGameInstance = {
	tournamentName: string
	level: 'Easy' | 'Medium' | 'Hard'
	participants: Map<HexString, any>
	bid: bigint
	stage: any
	durationMs: number
}

export type IGameCoins = {
	gold: number
	silver: number
}

////

export type IGameInstance = {
	level: IGameLevel // Уровень сложности
	playerAddress: HexString // Адрес игрока
	gold_coins: number // Количество золотых монет на карте
	silver_coins: number // Количество серебряных монет на карте
	start_time_ms: number // Время начала игры
	isClaimed: boolean // Флаг, который указывает на то, забрал ли игрок награду(клейм) или нет
}
