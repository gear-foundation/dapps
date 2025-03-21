import { TileMap } from '../types';

import { Vec2 } from './Vec2';

interface LimbAnimation {
  limb: 'left' | 'right';
  height: number;
  direction: 'up' | 'down';
}

export enum Direction {
  up = 'up',
  down = 'down',
  left = 'left',
  right = 'right',
}

export interface EnemyProps {
  x: number;
  y: number;
  zone: number;
  speed: number;
  mapData: TileMap;
}

interface DirectionHistoryItem {
  direction: Direction;
  timestamp: number;
}

export class Enemy {
  mapTiles: number[][];

  position: Vec2;
  zone: number;
  speed: number;
  direction: Direction;

  velocity: Vec2;
  rotation: number;
  scale: number;
  headRadius: number;
  torsoWidth: number;
  torsoHeight: number;
  legWidth: number;
  legHeight: number;
  armWidth: number;
  armHeight: number;
  walkSpeed: number;
  legs: LimbAnimation[];
  arms: LimbAnimation[];
  mapData: TileMap;

  previousDirections: DirectionHistoryItem[] = [];
  historyTimeThreshold: number = 15000;

  constructor({ x, y, zone, speed, mapData }: EnemyProps) {
    this.position = new Vec2(x, y);
    this.zone = zone;
    this.speed = speed;
    this.direction = Direction.up;

    this.scale = 0.4;
    this.position = new Vec2(x, y);
    this.velocity = new Vec2(0, speed * this.scale * -1);
    this.rotation = 0;
    this.mapData = mapData;

    // Scaling dimensions
    this.headRadius = 20 * this.scale;
    this.torsoWidth = 100 * this.scale;
    this.torsoHeight = 30 * this.scale;
    this.legWidth = 30 * this.scale;
    this.legHeight = 80 * this.scale;
    this.armWidth = 20 * this.scale;
    this.armHeight = 30 * this.scale;
    this.walkSpeed = speed * this.scale;

    this.previousDirections = [];

    // Initialize limb animations
    this.legs = [
      {
        limb: 'left',
        height: 0,
        direction: 'up',
      },
      {
        limb: 'right',
        height: 0,
        direction: 'down',
      },
    ];
    this.arms = [
      {
        limb: 'left',
        height: 0,
        direction: 'down',
      },
      {
        limb: 'right',
        height: 0,
        direction: 'up',
      },
    ];

    this.mapTiles = [];
    for (let i = 0; i < mapData.height; i++) {
      this.mapTiles[i] = [];
      for (let j = 0; j < mapData.width; j++) {
        const index = i * mapData.width + j;
        this.mapTiles[i][j] = mapData.layers[0].data[index];
      }
    }
  }

  getBounds() {
    return {
      x: this.position.x - this.torsoWidth / 2,
      y: this.position.y - this.torsoHeight * 2,
      width: this.torsoWidth,
      height: this.torsoHeight + this.legHeight,
    };
  }

  legAnimation(): void {
    this.legs.forEach((leg) => {
      const speedModifier = this.walkSpeed;
      if (leg.height <= this.legHeight * -1) {
        leg.direction = 'down';
      } else if (leg.height >= this.legHeight) {
        leg.direction = 'up';
      }

      leg.height += leg.direction === 'down' ? speedModifier : -speedModifier;
    });
  }

  armAnimation(): void {
    this.arms.forEach((arm) => {
      const speedModifier = this.walkSpeed;
      if (arm.height <= this.armHeight * -1) {
        arm.direction = 'down';
      } else if (arm.height >= this.armHeight) {
        arm.direction = 'up';
      }

      arm.height += arm.direction === 'down' ? speedModifier : -speedModifier;
    });
  }

  updateDirection(): void {
    const currentTileX = Math.floor(this.position.x / this.mapData.tilewidth);
    const currentTileY = Math.floor(this.position.y / this.mapData.tileheight);

    let availableDirections = this.getAvailableDirections(currentTileX, currentTileY);

    const currentTime = Date.now();
    availableDirections = availableDirections.filter(
      (dir) =>
        !this.previousDirections.some(
          (pd) => pd.direction === dir && currentTime - pd.timestamp < this.historyTimeThreshold,
        ),
    );

    if (availableDirections.length > 0) {
      const newDirections = availableDirections.filter((dir) => dir !== this.direction);
      const chosenDirection =
        newDirections.length > 0 ? newDirections[Math.floor(Math.random() * newDirections.length)] : this.direction;

      this.direction = chosenDirection;
      this.previousDirections.push({
        direction: chosenDirection,
        timestamp: currentTime,
      });

      this.cleanDirectionHistory();
    } else {
      this.velocity = new Vec2(0, 0);
    }
  }

  update(params: { mapData: TileMap; playerPosition?: Vec2 }): void {
    this.mapData = params.mapData;

    this.updateDirection();
    this.performMovement();
  }

  rotateEnemy() {
    let targetRotation = this.rotation;
    switch (this.direction) {
      case 'up':
        targetRotation = 0;
        break;
      case 'down':
        targetRotation = Math.PI;
        break;
      case 'left':
        targetRotation = -Math.PI / 2;
        break;
      case 'right':
        targetRotation = Math.PI / 2;
        break;
    }

    const rotationSpeed = 0.05;

    let rotationDifference = targetRotation - this.rotation;

    rotationDifference = ((rotationDifference + Math.PI) % (2 * Math.PI)) - Math.PI;

    if (rotationDifference > rotationSpeed) {
      this.rotation += rotationSpeed;
    } else if (rotationDifference < -rotationSpeed) {
      this.rotation -= rotationSpeed;
    } else {
      this.rotation = targetRotation;
    }
  }

  performMovement(_isPlayerInVision?: boolean) {
    const proposedPosition = new Vec2(this.position.x, this.position.y);

    switch (this.direction) {
      case Direction.up:
        proposedPosition.y -= this.speed;
        break;
      case Direction.down:
        proposedPosition.y += this.speed;
        break;
      case Direction.left:
        proposedPosition.x -= this.speed;
        break;
      case Direction.right:
        proposedPosition.x += this.speed;
        break;
    }

    if (!this.checkCollision(proposedPosition)) {
      this.position = proposedPosition;
      this.rotateEnemy();
    } else {
      this.chooseNewDirection();
    }

    this.legAnimation();
    this.armAnimation();
  }

  slideAlongWall(playerPosition: Vec2) {
    const directionToPlayer = Vec2.subtract(playerPosition, this.position).norm();
    const possibleDirections = [new Vec2(1, 0), new Vec2(-1, 0), new Vec2(0, 1), new Vec2(0, -1)];

    possibleDirections.sort((a, b) => {
      const dotA = Vec2.dot(a, directionToPlayer);
      const dotB = Vec2.dot(b, directionToPlayer);
      return dotB - dotA;
    });

    for (const dir of possibleDirections) {
      const newPos = Vec2.add(this.position, dir.scale(this.speed));
      if (!this.checkCollision(newPos)) {
        this.position = newPos;
        return;
      }
    }

    this.velocity = new Vec2(0, 0);
  }

  checkCollision(nextPosition: Vec2) {
    const bounds = this.getBounds();
    bounds.x += nextPosition.x - this.position.x;
    bounds.y += nextPosition.y - this.position.y;

    const left = Math.floor(bounds.x / this.mapData.tilewidth);
    const right = Math.ceil((bounds.x + bounds.width) / this.mapData.tilewidth) - 1;
    const top = Math.floor(bounds.y / this.mapData.tileheight);
    const bottom = Math.ceil((bounds.y + bounds.height) / this.mapData.tileheight) - 1;

    for (let y = top; y <= bottom; y++) {
      for (let x = left; x <= right; x++) {
        if (x >= 0 && x < this.mapData.width && y >= 0 && y < this.mapData.height) {
          const tileIndex = y * this.mapData.width + x;
          const tileValue = this.mapData.layers[0].data[tileIndex];
          if (tileValue === 1 || tileValue !== this.zone) {
            return true;
          }
        }
      }
    }

    if (
      bounds.x < 0 ||
      bounds.y < 0 ||
      bounds.x + bounds.width > this.mapData.width * this.mapData.tilewidth ||
      bounds.y + bounds.height > this.mapData.height * this.mapData.tileheight
    ) {
      return true;
    }

    return false;
  }

  chooseNewDirection(): void {
    const currentTileX = Math.floor(this.position.x / this.mapData.tilewidth);
    const currentTileY = Math.floor(this.position.y / this.mapData.tileheight);
    const availableDirections = this.getAvailableDirections(currentTileX, currentTileY);

    let filteredDirections = availableDirections.filter(
      (dir) =>
        !this.previousDirections.some(
          (pd) => pd.direction === dir && Date.now() - pd.timestamp < this.historyTimeThreshold,
        ),
    );

    if (filteredDirections.length === 0 && availableDirections.length > 0) {
      filteredDirections = availableDirections;
    }

    if (filteredDirections.length > 0) {
      const newDirection = filteredDirections[Math.floor(Math.random() * filteredDirections.length)];
      this.updateDirectionHistory(newDirection);
    }
  }

  updateDirectionHistory(newDirection: Direction): void {
    this.direction = newDirection;
    this.previousDirections.push({
      direction: newDirection,
      timestamp: Date.now(),
    });

    this.cleanDirectionHistory();
  }

  getAvailableDirections(currentTileX: number, currentTileY: number): Direction[] {
    const directions = [
      { dx: 0, dy: -1, dir: Direction.up },
      { dx: 1, dy: 0, dir: Direction.right },
      { dx: 0, dy: 1, dir: Direction.down },
      { dx: -1, dy: 0, dir: Direction.left },
    ];

    const availableDirections = [];

    for (const { dx, dy, dir } of directions) {
      let pathClear = true;
      for (let step = 1; step <= 5; step++) {
        const checkX = currentTileX + dx * step;
        const checkY = currentTileY + dy * step;

        if (checkX < 0 || checkX >= this.mapData.width || checkY < 0 || checkY >= this.mapData.height) {
          pathClear = false;
          break;
        }

        const tileIndex = checkY * this.mapData.width + checkX;
        const tile = this.mapData.layers[0].data[tileIndex];
        if (tile === 1 || tile !== this.zone) {
          pathClear = false;
          break;
        }
      }

      if (pathClear) {
        availableDirections.push(dir);
      }
    }

    return availableDirections;
  }

  cleanDirectionHistory(): void {
    const currentTime = Date.now();
    this.previousDirections = this.previousDirections.filter(
      (pd) => currentTime - pd.timestamp < this.historyTimeThreshold,
    );
  }
}
