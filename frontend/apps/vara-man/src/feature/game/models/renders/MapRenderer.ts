import { TileMap } from '../../types';
import { Vec2 } from '../Vec2';

class Tileset {
  image: HTMLImageElement;
  tileWidth: number;
  tileHeight: number;
  imageWidth: number;
  imageHeight: number;
  firstgid: number;
  tilecount: number;

  constructor(
    src: string,
    tileWidth: number,
    tileHeight: number,
    imageWidth: number,
    imageHeight: number,
    firstgid: number,
    tilecount: number,
  ) {
    this.image = new Image();
    this.image.src = src;
    this.tileWidth = tileWidth;
    this.tileHeight = tileHeight;
    this.imageWidth = imageWidth;
    this.imageHeight = imageHeight;
    this.firstgid = firstgid;
    this.tilecount = tilecount;
  }
}

export class MapRenderer {
  private static tilesets: Tileset[] = [];
  private static loadedImages: { [key: string]: HTMLImageElement } = {};

  public static async initTilesets(mapData: TileMap) {
    this.tilesets = mapData.tilesets.map(
      (tileset) =>
        new Tileset(
          tileset.image,
          tileset.tilewidth,
          tileset.tileheight,
          tileset.imagewidth,
          tileset.imageheight,
          tileset.firstgid,
          tileset.tilecount,
        ),
    );

    await Promise.all(
      this.tilesets.map((tileset) => {
        if (!this.loadedImages[tileset.image.src]) {
          return new Promise((resolve) => {
            tileset.image.onload = () => {
              this.loadedImages[tileset.image.src] = tileset.image;
              resolve(true);
            };
          });
        }
        return Promise.resolve(true);
      }),
    );
  }

  public static render(context: CanvasRenderingContext2D, mapData: TileMap) {
    const tileLayer = mapData.layers.find((layer) => layer.name === 'main');

    if (!tileLayer || !tileLayer.visible) {
      return;
    }

    const { width, height, data } = tileLayer;

    for (let y = 0; y < height; y++) {
      for (let x = 0; x < width; x++) {
        const tileIndex = data[y * width + x] - 1;
        if (tileIndex < 0) continue;

        for (const tileset of this.tilesets) {
          if (tileIndex < (tileset.imageWidth / tileset.tileWidth) * (tileset.imageHeight / tileset.tileHeight)) {
            const cols = tileset.imageWidth / tileset.tileWidth;
            const tx = (tileIndex % cols) * tileset.tileWidth;
            const ty = Math.floor(tileIndex / cols) * tileset.tileHeight;
            context.drawImage(
              tileset.image,
              tx,
              ty,
              tileset.tileWidth,
              tileset.tileHeight,
              x * mapData.tilewidth,
              y * mapData.tileheight,
              mapData.tilewidth,
              mapData.tileheight,
            );
            break;
          }
        }
      }
    }

    this.renderImageLayer(context, mapData);
    this.renderCoins(context, mapData);
  }

  public static renderCoins(context: CanvasRenderingContext2D, mapData: TileMap) {
    const coinLayer = mapData.layers.find((layer) => layer.name === 'coins');
    if (!coinLayer || !coinLayer.visible) {
      return;
    }

    const { width, height, data } = coinLayer;

    for (let y = 0; y < height; y++) {
      for (let x = 0; x < width; x++) {
        const tileIndex = data[y * width + x];
        if (tileIndex > 0) {
          const tileset = this.tilesets.find(
            (ts) => tileIndex >= ts.firstgid && tileIndex < ts.firstgid + ts.tilecount,
          );
          if (!tileset) continue;

          const localTileIndex = tileIndex - tileset.firstgid;
          const cols = tileset.imageWidth / tileset.tileWidth;
          const tx = (localTileIndex % cols) * tileset.tileWidth;
          const ty = Math.floor(localTileIndex / cols) * tileset.tileHeight;

          context.drawImage(
            tileset.image,
            tx,
            ty,
            tileset.tileWidth,
            tileset.tileHeight,
            x * mapData.tilewidth,
            y * mapData.tileheight,
            mapData.tilewidth,
            mapData.tileheight,
          );
        }
      }
    }
  }

  static renderFogOfWar(context: CanvasRenderingContext2D, playerPosition: Vec2, visionRadius: number) {
    context.fillStyle = 'white';

    context.fillRect(0, 0, context.canvas.width, context.canvas.height);

    const gradient = context.createRadialGradient(
      playerPosition.x,
      playerPosition.y,
      visionRadius * 0.8,
      playerPosition.x,
      playerPosition.y,
      visionRadius * 0.9,
    );
    gradient.addColorStop(0, 'rgba(255, 255, 255, 0)');
    gradient.addColorStop(0.8, 'rgba(255, 255, 255, 0.9)');
    gradient.addColorStop(1, 'rgba(255, 255, 255, 1)');

    context.globalCompositeOperation = 'destination-out';
    context.beginPath();
    context.arc(playerPosition.x, playerPosition.y, visionRadius, 0, Math.PI * 2, true);

    context.fill();

    context.closePath();

    context.globalCompositeOperation = 'source-over';

    context.fillStyle = gradient;

    context.beginPath();

    context.arc(playerPosition.x, playerPosition.y, visionRadius, 0, Math.PI * 2, true);

    context.strokeStyle = 'white';
    context.stroke();
    context.closePath();
    context.fill();
  }

  public static renderImageLayer(context: CanvasRenderingContext2D, mapData: TileMap) {
    const imageLayer = mapData.layers.find((layer) => layer.type === 'imagelayer');

    if (!imageLayer || !imageLayer.visible) {
      return;
    }

    if (imageLayer.image) {
      if (!this.loadedImages[imageLayer.image]) {
        const image = new Image();
        image.src = imageLayer.image;
        image.onload = () => {
          context.drawImage(image, 0, 0);
          if (imageLayer.image) {
            this.loadedImages[imageLayer.image] = image;
          }
        };
      } else {
        context.drawImage(this.loadedImages[imageLayer.image], 0, 0);
      }
    }
  }
}
