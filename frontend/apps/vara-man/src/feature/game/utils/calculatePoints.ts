import { IGameCoins } from '@/app/types/game';

export const calculatePoints = (coins: IGameCoins, configState: Config, level: Level) => {
  const loverCaseLevel = level.toLowerCase() as 'easy' | 'medium' | 'hard';
  const pointsPerGoldCoin = configState[`points_per_gold_coin_${loverCaseLevel}`];
  const pointsPerSilverCoin = configState[`points_per_silver_coin_${loverCaseLevel}`];

  const points =
    Math.floor(Number(pointsPerGoldCoin) * coins.gold) + Math.floor(Number(pointsPerSilverCoin) * coins.silver);

  return points;
};
