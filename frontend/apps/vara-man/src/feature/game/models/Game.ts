import { Character } from './Character';
import { CharacterRenderer } from './renders/CharacterRenderer';
import { MapRenderer } from './renders/MapRenderer';
import { EnemyRenderer } from './renders/EnemyRenderer';
import { EnemyWithVision } from './EnemyWithVision';
import { findEnemyStartPositions } from '../utils/findEnemyStartPositions';
import { findCharacterStartPosition } from '../utils/findCharacterStartPosition';
import { IGameLevel } from '@/app/types/game';
import { TileMap } from '../types';
import { HEIGHT_CANVAS, WIDTH_CANVAS, gameLevels } from '../consts';

export class GameEngine {
  private context: CanvasRenderingContext2D;
  private fogContext: CanvasRenderingContext2D;

  private character: Character | undefined;
  private enemies: EnemyWithVision[] = [];
  private animationFrameId: number | null = null;
  private resizeTimeout: number | undefined;

  private isUp = false;
  private isDown = false;
  private isLeft = false;
  private isRight = false;
  private isShift = false;

  private incrementCoins: (coin: 'silver' | 'gold') => void;

  private lastUpdateTime: number = 0; // Добавлено для ограничения FPS
  private readonly frameDuration: number = 1000 / 60; // 60 FPS

  map: TileMap;
  level: IGameLevel;

  setGameOver = (gameOver: boolean) => {};
  gameOver = false;
  pause?: boolean;

  constructor(
    private canvas: HTMLCanvasElement,
    private canvasFog: HTMLCanvasElement,
    level: IGameLevel,
    incrementCoins: (coin: 'silver' | 'gold') => void,
    gameOver: boolean,
    setGameOver: (gameOver: boolean) => void,
    map: TileMap,
    pause?: boolean,
  ) {
    this.map = map;
    this.level = level;

    this.context = canvas.getContext('2d') as CanvasRenderingContext2D;
    this.fogContext = canvasFog.getContext('2d') as CanvasRenderingContext2D;
    this.setGameOver = setGameOver;
    this.gameOver = gameOver;
    this.pause = pause;
    this.incrementCoins = incrementCoins;
    this.init();
    this.resize();
  }

  init() {
    MapRenderer.initTilesets(this.map).then(() => {
      const startPosition = findCharacterStartPosition(this.map);
      const enemyStartPositions = findEnemyStartPositions(this.map);

      if (startPosition) {
        this.character = new Character(startPosition.x, startPosition.y, true, this.map, this.incrementCoins, () =>
          this.setGameOver(true),
        );

        this.initEventListeners();

        this.resize();
      } else {
        console.error('The character starting position was not found.');
      }

      const levelData = gameLevels.find((l) => {
        return l.level === this.level;
      });

      enemyStartPositions.forEach(({ position, zone }) => {
        if (this.character) {
          const enemy = new EnemyWithVision(
            {
              x: position.x,
              y: position.y,
              zone: zone,
              speed: levelData!.speed,
              mapData: this.map,
            },
            this.character.position,
            levelData!.visionEnemy,
          );
          this.enemies.push(enemy);
        }
      });

      CharacterRenderer.loadCloakImage('./cloak.svg')
        .then((img) => {
          CharacterRenderer.cloakImage = img;
          this.update();
        })
        .catch((error) => {
          console.error(error);
          this.update();
        });
    });
  }

  public getCharacter(): Character | undefined {
    return this.character;
  }

  resize = () => {
    const dpr = Math.min(window.devicePixelRatio, 1.5);
    const width = WIDTH_CANVAS * dpr;
    const height = HEIGHT_CANVAS * dpr;

    this.canvas.width = width;
    this.canvas.height = height;
    this.canvasFog.width = width;
    this.canvasFog.height = height;

    this.context.scale(dpr, dpr);
    this.fogContext.scale(dpr, dpr);

    this.render();
  };

  private handleResize = () => {
    if (this.resizeTimeout) {
      clearTimeout(this.resizeTimeout);
    }
    this.resizeTimeout = window.setTimeout(() => {
      this.resize();
    }, 200);
  };

  private initEventListeners() {
    window.addEventListener('keydown', this.handleKeyDown);
    window.addEventListener('keyup', this.handleKeyUp);
    window.addEventListener('resize', this.handleResize);
  }

  public handleKeyDown = (event: { keyCode: number }) => {
    switch (event.keyCode) {
      case 38: // Arrow Up
      case 87: // W
        this.isUp = true;
        break;
      case 40: // Arrow Down
      case 83: // S
        this.isDown = true;
        break;
      case 37: // Arrow Left
      case 65: // A
        this.isLeft = true;
        break;
      case 39: // Arrow Right
      case 68: // D
        this.isRight = true;
        break;
      case 16: // Shift
        this.isShift = true;
        break;
    }
  };

  public handleKeyUp = (event: { key: string }) => {
    switch (event.key) {
      case 'ArrowUp':
      case 'w':
      case 'W':
        this.isUp = false;
        break;
      case 'ArrowDown':
      case 's':
      case 'S':
        this.isDown = false;
        break;
      case 'ArrowLeft':
      case 'a':
      case 'A':
        this.isLeft = false;
        break;
      case 'ArrowRight':
      case 'd':
      case 'D':
        this.isRight = false;
        break;
      case 'Shift':
        this.isShift = false;
        break;
    }
  };

  update = () => {
    if (this.gameOver) {
      this.cleanup();
      return;
    }

    const now = performance.now();
    const deltaTime = now - this.lastUpdateTime;

    if (deltaTime >= this.frameDuration) {
      this.lastUpdateTime = now - (deltaTime % this.frameDuration);

      if (!this.pause && this.character) {
        this.character.updateMovement(this.isLeft, this.isRight, this.isUp, this.isDown, this.isShift);

        this.enemies.forEach((enemy) => {
          enemy.update({ mapData: this.map, playerPosition: this.character!.position });
        });

        if (this.checkCollisions()) {
          this.setGameOver(true);
          return;
        }

        this.context.clearRect(0, 0, this.canvas.width, this.canvas.height);
        this.render();
      }
    }

    this.animationFrameId = requestAnimationFrame(this.update);
  };

  render() {
    if (!this.character) {
      requestAnimationFrame(this.update);
      return;
    }

    let offsetX = 0;
    let offsetY = 0;
    if (window.innerWidth < 768) {
      offsetX = window.innerWidth / 2.8 - this.character.position.x;
      offsetY = window.innerHeight / 4 - this.character.position.y;

      this.context.save();
      this.context.translate(offsetX, offsetY);
    }

    this.context.fillStyle = '#000000ad';
    this.context.fillRect(0, 0, WIDTH_CANVAS, HEIGHT_CANVAS);

    MapRenderer.render(this.context, this.map);
    CharacterRenderer.render(this.context, this.character);

    this.enemies.forEach((enemy) => EnemyRenderer.render(this.context, enemy));

    this.context.restore();

    if (this.level === 'Hard') {
      this.fogContext.save();
      this.fogContext.translate(offsetX, offsetY);
      const radiusFogOfWar = window.innerWidth < 768 ? 120 : 150;
      MapRenderer.renderFogOfWar(this.fogContext, this.character.position, radiusFogOfWar);
      this.fogContext.restore();
    }
  }

  public cleanup() {
    if (this.animationFrameId !== null) {
      cancelAnimationFrame(this.animationFrameId);
      this.animationFrameId = null;
    }

    window.removeEventListener('keydown', this.handleKeyDown);
    window.removeEventListener('keyup', this.handleKeyUp);
    window.removeEventListener('resize', this.handleResize);

    if (this.resizeTimeout) {
      clearTimeout(this.resizeTimeout);
    }
  }

  checkCollisions() {
    if (!this.character) return false;

    const characterBounds = this.character.getBounds();

    for (const enemy of this.enemies) {
      const enemyBounds = enemy.getBounds();

      if (
        characterBounds.x < enemyBounds.x + enemyBounds.width &&
        characterBounds.x + characterBounds.width > enemyBounds.x &&
        characterBounds.y < enemyBounds.y + enemyBounds.height &&
        characterBounds.y + characterBounds.height > enemyBounds.y
      ) {
        return true;
      }
    }

    return false;
  }

  public updateGameOver = (gameOver: boolean) => {
    this.gameOver = gameOver;
    if (gameOver) {
      this.cleanup();
    }
  };

  public updatePause = () => {
    this.pause = false;
  };
}
