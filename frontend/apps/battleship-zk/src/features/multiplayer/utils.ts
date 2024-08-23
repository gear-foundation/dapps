import { MultipleGameState } from '@/app/utils/sails/lib/lib';

export const getIsPlacementStatus = (game: MultipleGameState | null | undefined) =>
  Object.keys(game?.status || {})[0] === 'verificationPlacement';
