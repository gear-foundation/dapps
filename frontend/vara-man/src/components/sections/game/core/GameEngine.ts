import Character from './Character'
import Enemy from './Enemy'
import TileMap from './TileMap'

interface GameActions {
  incrementCoins: (coinType: 'silver' | 'gold') => void
  setGameOver: (_: boolean) => void
}

class GameEngine {
  private readonly VELOCITY = 2
  private readonly TILE_SIZE = 32

  private canvas: HTMLCanvasElement | null
  private tileMap: TileMap
  private character: Character | undefined
  private enemies: Enemy[] = []
  private gameActions: GameActions
  private isStopGame: boolean
  private timer: number
  private timerInterval: NodeJS.Timeout | null = null
  private animationId: number | null = null

  constructor(
    canvas: HTMLCanvasElement,
    gameActions: GameActions,
    timer: number,
  ) {
    this.canvas = canvas
    this.tileMap = new TileMap(this.TILE_SIZE, canvas)
    this.tileMap.initialize().then(() => {
      this.character = this.tileMap.getCharacter(this.VELOCITY)
      this.enemies = []
      this.enemies = this.tileMap.getEnemies(this.VELOCITY)
    })
    this.gameActions = gameActions
    this.isStopGame = false
    this.timer = timer

    this.timerInterval = setInterval(() => {
      this.timer -= 1
    }, 1000)
  }

  startGameLoop() {
    if (!this.animationId) {
      const animate = () => {
        if (!this.isStopGame) {
          this.gameLoop()
          this.animationId = requestAnimationFrame(animate)
        }
      }

      // Start the animation loop
      this.animationId = requestAnimationFrame(animate)
    }
  }

  // Method to stop the game loop
  stopGameLoop() {
    if (this.animationId) {
      cancelAnimationFrame(this.animationId)
      this.animationId = null
    }
  }

  gameStart() {
    this.isStopGame = false
    this.startGameLoop()
  }

  gameOver() {
    this.isStopGame = true
    this.stopGameLoop()
  }

  gameLoop() {
    if (this.canvas) {
      const ctx = this.canvas.getContext('2d')

      if (!ctx || !this.character) return

      ctx.imageSmoothingEnabled = false
      ctx.clearRect(0, 0, this.canvas.width, this.canvas.height)

      this.tileMap.draw(ctx)
      this.character.draw(ctx, this.pause(), this.enemies)
      this.enemies.forEach((enemy) => enemy.draw(ctx, this.pause()))

      this.drawGameEnd()

      if (this.tileMap && this.tileMap.isCoinEaten()) {
        const coin = this.tileMap.getCoinEaten()
        coin && this.gameActions.incrementCoins(coin)
      }
    }
  }

  endGame(animationId: number) {
    this.clearTimerInterval()
    cancelAnimationFrame(animationId)
    console.log('cancelAnimationFrame(animationId)', animationId)
  }

  setPause(isStart: boolean) {
    this.isStopGame = isStart
  }

  pause() {
    return this.isStopGame
  }

  drawGameEnd() {
    const isCollideWith = this.enemies.some((enemy) =>
      enemy.collideWith(this.character)
    )

    if (isCollideWith || this.timer <= 0 || this.tileMap.didWin()) {
      this.setPause(true)
      this.gameActions.setGameOver(true)
      this.clearTimerInterval()
    }
  }

  setCanvasSize() {
    this.canvas && this.tileMap.setCanvasSize(this.canvas)
  }

  clearTimerInterval() {
    if (this.timerInterval) {
      clearInterval(this.timerInterval)
      this.timerInterval = null
    }
  }
}

export default GameEngine
