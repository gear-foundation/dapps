import { IGameCoins, IGameConfig, IGameLevel } from '@/app/types/game'

export const calculatePoints = (
	coins: IGameCoins,
	configState: IGameConfig,
	level: 'Easy' | 'Medium' | 'Hard'
) => {
	const pointsPerGoldCoin = configState[`pointsPerGoldCoin${level}`]
	const pointsPerSilverCoin = configState[`pointsPerSilverCoin${level}`]

	const points =
		Math.floor(Number(pointsPerGoldCoin) * coins.gold) +
		Math.floor(Number(pointsPerSilverCoin) * coins.silver)

	return points
}
