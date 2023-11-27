import { TamagotchiAvatarAge } from '../types/tamagotchi';

const MS_MULTIPLIER = 1000;
const S_MULTIPLIER = 60;

export const getTamagotchiAgeDiff = (value: number): TamagotchiAvatarAge => {
  const milliseconds = Date.now() - value;
  const minutes = Math.floor(milliseconds / (MS_MULTIPLIER * S_MULTIPLIER));

  return minutes > 60 ? 'old' : minutes > 20 ? 'adult' : 'baby';
};
