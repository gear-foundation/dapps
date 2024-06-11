import { HexString } from '@polkadot/util/types'

export type IGameState = {
	admins: string[]
	games: IGame[]
	players: IPlayer[]
	config: IGameConfig
}

export type IGameStatus = 'Paused' | 'Started'

export type IPlayer = [HexString, IPlayerInfo]
export type IGame = [
	string,
	{
		admin: HexString
		tournamentName: string
		level: 'Easy' | 'Medium' | 'Hard'
		participants: [
			[
				HexString,
				{
					name: string
					points: string
					time: string
				}
			]
		]
		bid: bigint
		stage: any
		durationMs: string
	}
]

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

// interface SingleGameState {
// 	type: 'SingleGame'
// 	SingleGame: ISingleGameInstance
// }

// interface TournamentState {
// 	type: 'Tournament'
// 	TournamentGame: ITournamentGameInstance
// }

// export type GameState = TournamentState

export type ISingleGameInstance = [
	{
		level: 'Easy' | 'Medium' | 'Hard'
		points: string
		startTime: string
		gameOver: boolean
	},
	string
]

export type ITournamentGameInstance = [
	{
		admin: HexString
		tournamentName: string
		level: 'Easy' | 'Medium' | 'Hard'
		participants: [
			[
				HexString,
				{
					name: string
					points: string
					time: string
				}
			]
		]
		bid: bigint
		stage: Finished | any
		durationMs: string
	},
	string
]

export type IGameCoins = {
	gold: number
	silver: number
}

type Finished = {
	Finished: string[]
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

export type TileMap = {
	compressionlevel: number
	height: number
	infinite: boolean
	layers: Array<{
		data: number[]
		height: number
		id: number
		name: string
		opacity: number
		type: string
		visible: boolean
		width: number
		x: number
		y: number
		image?: string
	}>
	nextlayerid: number
	nextobjectid: number
	orientation: string
	renderorder: string
	tiledversion: string
	tileheight: number
	tilesets: Array<{
		columns: number
		firstgid: number
		image: string
		imageheight: number
		imagewidth: number
		margin: number
		name: string
		spacing: number
		tilecount: number
		tileheight: number
		tilewidth: number
	}>
	tilewidth: number
	type: string
	version: string | number
	width: number
}
