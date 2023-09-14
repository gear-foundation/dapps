import MovingDirection from './MovingDirection'
import Zombie from '@/assets/images/game/zombie.svg'

export default class Enemy {
  x: number
  y: number
  tileSize: number
  velocity: number
  tileMap: any

  movingDirection: number
  directionTimerDefault: number
  directionTimer: number
  scaredAboutToExpireTimerDefault: number
  scaredAboutToExpireTimer: number
  normalGhost!: HTMLImageElement
  scaredGhost!: HTMLImageElement
  scaredGhost2!: HTMLImageElement
  image!: HTMLImageElement

  private directionChangeInterval: number = 500
  private lastDirectionChangeTime: number = 0

  constructor(
    x: number,
    y: number,
    tileSize: number,
    velocity: number,
    tileMap: any
  ) {
    this.x = x
    this.y = y
    this.tileSize = tileSize
    this.velocity = velocity
    this.tileMap = tileMap

    this.loadImages()

    this.movingDirection = Math.floor(
      Math.random() * Object.keys(MovingDirection).length
    )

    this.directionTimerDefault = this.random(10, 25)
    this.directionTimer = this.directionTimerDefault

    this.scaredAboutToExpireTimerDefault = 10
    this.scaredAboutToExpireTimer = this.scaredAboutToExpireTimerDefault
  }

  draw(ctx: CanvasRenderingContext2D, pause: boolean) {
    if (!pause) {
      // Call the chooseDirectionBasedOnAdjacentCells() method to get a direction
      const newDirection = this.chooseDirectionBasedOnAdjacentCells()

      // If the new direction is different from the current one, update the direction
      if (newDirection !== this.movingDirection) {
        this.movingDirection = newDirection
      }

      this.move()
    }
    this.setImage(ctx)
  }

  collideWith(character: any) {
    const size = this.tileSize / 2
    if (
      this.x < character.x + size &&
      this.x + size > character.x &&
      this.y < character.y + size &&
      this.y + size > character.y
    ) {
      return true
    } else {
      return false
    }
  }

  private setImage(ctx: CanvasRenderingContext2D) {
    ctx.drawImage(this.image, this.x - 5, this.y - 20)
  }

  private move() {
    const stepSize = this.velocity

    let newX = this.x
    let newY = this.y

    for (let i = 0; i < stepSize; i++) {
      const didCollideWithEnvironment = this.tileMap.didCollideWithEnvironment(
        newX,
        newY,
        this.movingDirection
      )

      if (!didCollideWithEnvironment) {
        switch (this.movingDirection) {
          case MovingDirection.up:
            newY -= 1
            break
          case MovingDirection.down:
            newY += 1
            break
          case MovingDirection.left:
            newX -= 1
            break
          case MovingDirection.right:
            newX += 1
            break
        }
      }
    }

    this.x = newX
    this.y = newY
  }

  getCurrentCell(): { row: number; column: number } {
    const currentRow = Math.floor(this.y / this.tileSize)
    const currentColumn = Math.floor(this.x / this.tileSize)
    return { row: currentRow, column: currentColumn }
  }

  getAdjacentCells(): { row: number; column: number }[] {
    const currentCell = this.getCurrentCell()
    const adjacentCells: { row: number; column: number }[] = []

    adjacentCells.push({ row: currentCell.row - 1, column: currentCell.column })
    adjacentCells.push({ row: currentCell.row + 1, column: currentCell.column })
    adjacentCells.push({ row: currentCell.row, column: currentCell.column - 1 })
    adjacentCells.push({ row: currentCell.row, column: currentCell.column + 1 })

    return adjacentCells
  }

  chooseDirectionBasedOnAdjacentCells(): number {
    const currentCell = this.getCurrentCell()
    const adjacentCells = this.getAdjacentCells()

    const currentTime = Date.now()

    // Check if enough time has passed since the last direction change
    if (
      currentTime - this.lastDirectionChangeTime >=
      this.directionChangeInterval
    ) {
      const availableDirections: number[] = []

      for (const cell of adjacentCells) {
        const cellValue = this.tileMap.initialMap[cell.row][cell.column]
        if (cellValue === 0 || cellValue === 5) {
          const direction = this.calculateDirectionToCell(currentCell, cell)
          availableDirections.push(direction)
        }
      }

      if (
        Number.isInteger(this.x / this.tileSize) &&
        Number.isInteger(this.y / this.tileSize)
      ) {
        // If there are available directions, choose a random one
        if (availableDirections.length > 0) {
          const randomIndex = Math.floor(
            Math.random() * availableDirections.length
          )
          const newDirection = availableDirections[randomIndex]

          this.movingDirection = newDirection

          // Update the time of the last direction change
          this.lastDirectionChangeTime = currentTime
        }
      }
    }

    return this.movingDirection
  }

  calculateDirectionToCell(
    currentCell: { row: number; column: number },
    targetCell: { row: number; column: number }
  ): number {
    // Calculate the direction to the target cell relative to the current cell
    if (targetCell.row < currentCell.row) {
      return MovingDirection.up
    } else if (targetCell.row > currentCell.row) {
      return MovingDirection.down
    } else if (targetCell.column < currentCell.column) {
      return MovingDirection.left
    } else if (targetCell.column > currentCell.column) {
      return MovingDirection.right
    }

    // If the cells are at the same position, return the current direction
    return this.movingDirection
  }

  private random(min: number, max: number) {
    return Math.floor(Math.random() * (max - min + 1)) + min
  }

  private loadImages() {
    this.normalGhost = new Image()
    this.normalGhost.src = Zombie

    this.image = this.normalGhost
  }
}
