import { Level } from '@/app/utils';

import EasyMap from '../assets/map/map-easy.json';
import HardMap from '../assets/map/map-hard.json';
import MediumMap from '../assets/map/map-medium.json';
import { TileMap } from '../types';

const maps: Record<Level, TileMap> = {
  Easy: EasyMap as TileMap,
  Medium: MediumMap as TileMap,
  Hard: HardMap as TileMap,
};

export const findMapLevel = (level: Level): TileMap => {
  const map = maps[level];

  if (!map) {
    throw new Error(`Map for level "${level}" not found.`);
  }

  const mapCopy: TileMap = JSON.parse(JSON.stringify(map)) as TileMap;

  return mapCopy;
};
