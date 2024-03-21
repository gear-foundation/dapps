import { atom } from 'jotai'
import { IGameCoins } from '@/app/types/game'

export const COINS = atom<IGameCoins>({
	gold: 0,
	silver: 0,
})

export const GAME_OVER = atom<boolean>(false)

export const gameLevels = [
	{
		level: 'Easy',
		speed: 1,
		enemies: 4,
		visionEnemy: 0,
	},
	{
		level: 'Medium',
		speed: 1,
		enemies: 4,
		visionEnemy: 10,
	},
	{
		level: 'Hard',
		speed: 1.5,
		enemies: 4,
		visionEnemy: 50,
	},
]

export const MS_TIME_GAME_OVER = atom(0)

export const PRIZE_POOL = atom(0)
