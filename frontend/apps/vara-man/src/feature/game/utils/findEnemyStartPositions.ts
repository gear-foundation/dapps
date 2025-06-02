import { Vec2 } from '../models/Vec2';
import { TileMap } from '../types';

function determineEnemyZone(x: number, y: number, mapData: TileMap): number {
  const tileLayer = mapData.layers.find((layer) => layer.type === 'tilelayer');
  if (!tileLayer) return -1;

  const offsets = [
    { dx: -1, dy: -1 },
    { dx: 0, dy: -1 },
    { dx: 1, dy: -1 },
    { dx: -1, dy: 0 },
    { dx: 1, dy: 0 },
    { dx: -1, dy: 1 },
    { dx: 0, dy: 1 },
    { dx: 1, dy: 1 },
  ];

  const zoneCounts: { [key: number]: number } = {};

  offsets.forEach((offset) => {
    const nx = x + offset.dx;
    const ny = y + offset.dy;

    if (nx >= 0 && nx < tileLayer.width && ny >= 0 && ny < tileLayer.height) {
      const tileIndex = ny * tileLayer.width + nx;
      const tileValue = tileLayer.data[tileIndex];

      if (tileValue !== 1 && tileValue > 0) {
        zoneCounts[tileValue] = (zoneCounts[tileValue] || 0) + 1;
      }
    }
  });

  let maxZone = -1,
    maxCount = 0;
  for (const zone in zoneCounts) {
    if (zoneCounts[zone] > maxCount) {
      maxCount = zoneCounts[zone];
      maxZone = parseInt(zone);
    }
  }

  return maxZone;
}

export function findEnemyStartPositions(mapData: TileMap): { position: Vec2; zone: number }[] {
  const tileLayer = mapData.layers.find((layer) => layer.name === 'main');
  if (!tileLayer) return [];

  const positions: { position: Vec2; zone: number }[] = [];

  for (let y = 0; y < tileLayer.height; y++) {
    for (let x = 0; x < tileLayer.width; x++) {
      const tileIndex = tileLayer.data[y * tileLayer.width + x];
      if (tileIndex === 5) {
        const zone = determineEnemyZone(x, y, mapData);
        positions.push({
          position: new Vec2(x * mapData.tilewidth, y * mapData.tileheight),
          zone: zone,
        });

        tileLayer.data[y * tileLayer.width + x] = zone;
      }
    }
  }
  return positions;
}
