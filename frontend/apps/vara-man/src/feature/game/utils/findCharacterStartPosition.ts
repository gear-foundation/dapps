import { Vec2 } from '../models/Vec2';
import { TileMap } from '../types';

export function findCharacterStartPosition(mapData: TileMap): Vec2 | null {
  const tileLayer = mapData.layers.find((layer) => layer.name === 'main');
  if (!tileLayer) return null;

  for (let y = 0; y < tileLayer.height; y++) {
    for (let x = 0; x < tileLayer.width; x++) {
      const tileIndex = tileLayer.data[y * tileLayer.width + x];

      if (tileIndex === 6) {
        tileLayer.data[y * tileLayer.width + x] = 4;
        return new Vec2(x * mapData.tilewidth, y * mapData.tileheight);
      }
    }
  }

  return null;
}
