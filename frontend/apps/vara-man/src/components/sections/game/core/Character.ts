import MovingDirection from './MovingDirection';

import Tamagochi from '@/assets/images/game/tamagochi.svg';
import TileMap from './TileMap';

export default class Character {
  x: number;
  y: number;
  tileSize: number;
  velocity: number;
  tileMap: TileMap;

  currentMovingDirection: any;
  requestedMovingDirection: any;

  characterAnimationTimerDefault: number;
  characterAnimationTimer: number | null;

  characterRotation: any;
  powerDotActive: boolean;
  powerDotAboutToExpire: boolean;
  timers: number[];

  madeFirstMove: boolean | undefined;

  Rotation = MovingDirection;

  characterImages!: HTMLImageElement[];
  characterImageIndex!: number;

  private readonly CHARACTER_IMAGE_OFFSET_X = -10;
  private readonly CHARACTER_IMAGE_OFFSET_Y = -15;

  constructor(x: number, y: number, tileSize: number, velocity: number, tileMap: any) {
    this.x = x;
    this.y = y;
    this.tileSize = tileSize;
    this.velocity = velocity;
    this.tileMap = tileMap;

    this.currentMovingDirection = null;
    this.requestedMovingDirection = null;

    this.characterAnimationTimerDefault = 10;
    this.characterAnimationTimer = null;

    this.characterRotation = this.Rotation.right;
    this.powerDotActive = false;
    this.powerDotAboutToExpire = false;
    this.timers = [];

    document.addEventListener('keydown', this.keydown);

    this.loadCharacterImages();
  }

  draw(ctx: CanvasRenderingContext2D, pause: boolean, enemies: any[]) {
    if (!pause) {
      this.move();
      this.animate();
    }
    this.eatDot();

    const size = this.tileSize;

    ctx.save();
    ctx.translate(this.x + size, this.y + size);
    // ctx.rotate((this.characterRotation * 90 * Math.PI) / 180)

    ctx.drawImage(
      this.characterImages[this.characterImageIndex],
      -size + this.CHARACTER_IMAGE_OFFSET_X,
      -size + this.CHARACTER_IMAGE_OFFSET_Y,
    );

    ctx.restore();
  }

  private loadCharacterImages() {
    const characterImage1 = new Image();
    characterImage1.src = Tamagochi;

    const characterImage2 = new Image();
    characterImage2.src = Tamagochi;

    const characterImage3 = new Image();
    characterImage3.src = Tamagochi;

    const characterImage4 = new Image();
    characterImage4.src = Tamagochi;
    this.characterImages = [characterImage1, characterImage2, characterImage3, characterImage4];

    this.characterImageIndex = 0;
  }

  private keydown = (event: KeyboardEvent) => {
    //up
    if (event.keyCode == 38) {
      if (this.currentMovingDirection == MovingDirection.down) this.currentMovingDirection = MovingDirection.up;
      this.requestedMovingDirection = MovingDirection.up;
      this.madeFirstMove = true;
    }
    //down
    if (event.keyCode == 40) {
      if (this.currentMovingDirection == MovingDirection.up) this.currentMovingDirection = MovingDirection.down;
      this.requestedMovingDirection = MovingDirection.down;
      this.madeFirstMove = true;
    }
    //left
    if (event.keyCode == 37) {
      if (this.currentMovingDirection == MovingDirection.right) this.currentMovingDirection = MovingDirection.left;
      this.requestedMovingDirection = MovingDirection.left;
      this.madeFirstMove = true;
    }
    //right
    if (event.keyCode == 39) {
      if (this.currentMovingDirection == MovingDirection.left) this.currentMovingDirection = MovingDirection.right;
      this.requestedMovingDirection = MovingDirection.right;
      this.madeFirstMove = true;
    }
  };

  private move() {
    const isAtIntegerPosition = Number.isInteger(this.x / this.tileSize) && Number.isInteger(this.y / this.tileSize);

    if (this.currentMovingDirection !== this.requestedMovingDirection && isAtIntegerPosition) {
      const nextX = Math.floor(this.x / this.tileSize) * this.tileSize;
      const nextY = Math.floor(this.y / this.tileSize) * this.tileSize;

      if (!this.tileMap.didCollideWithEnvironment(nextX, nextY, this.requestedMovingDirection)) {
        this.currentMovingDirection = this.requestedMovingDirection;
      }
    }

    if (this.tileMap.didCollideWithEnvironment(this.x, this.y, this.currentMovingDirection)) {
      this.characterAnimationTimer = null;
      this.characterImageIndex = 1;
      return;
    }

    if (this.currentMovingDirection != null && this.characterAnimationTimer == null) {
      this.characterAnimationTimer = this.characterAnimationTimerDefault;
    }

    const stepSize = this.velocity;
    let newX = this.x;
    let newY = this.y;

    for (let i = 0; i < stepSize; i++) {
      const didCollideWithEnvironment = this.tileMap.didCollideWithEnvironment(newX, newY, this.currentMovingDirection);

      if (!didCollideWithEnvironment) {
        switch (this.currentMovingDirection) {
          case MovingDirection.up:
            newY -= 1;
            break;
          case MovingDirection.down:
            newY += 1;
            break;
          case MovingDirection.left:
            newX -= 1;
            break;
          case MovingDirection.right:
            newX += 1;
            break;
        }
      }
    }

    this.x = newX;
    this.y = newY;
  }

  private animate() {
    if (this.characterAnimationTimer == null) {
      return;
    }

    this.characterAnimationTimer--;
    if (this.characterAnimationTimer === 0) {
      this.characterAnimationTimer = this.characterAnimationTimerDefault;
      this.characterImageIndex = (this.characterImageIndex + 1) % this.characterImages.length;
    }
  }

  private eatDot() {
    this.tileMap.eatDot(this.x, this.y);
  }
}
