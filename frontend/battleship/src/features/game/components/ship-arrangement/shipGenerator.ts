type Entity = 'Empty' | 'Ship'

function checkEmptyField(
  board: Entity[],
  size: number,
  row: number,
  col: number,
  isVertical: boolean
): boolean {
  const boardSize = Math.sqrt(board.length)

  if (isVertical) {
    for (let i = row; i < row + size; i++) {
      if (
        i >= boardSize ||
        board[i * boardSize + col] === 'Ship' ||
        (col > 0 && board[i * boardSize + col - 1] === 'Ship') ||
        (col < boardSize - 1 && board[i * boardSize + col + 1] === 'Ship')
      ) {
        return false
      }
    }
  } else {
    for (let i = col; i < col + size; i++) {
      if (
        i >= boardSize ||
        board[row * boardSize + i] === 'Ship' ||
        (row > 0 && board[(row - 1) * boardSize + i] === 'Ship') ||
        (row < boardSize - 1 && board[(row + 1) * boardSize + i] === 'Ship')
      ) {
        return false
      }
    }
  }
  return true
}

function placeShip(
  board: Entity[],
  size: number,
  row: number,
  col: number,
  isVertical: boolean
): number[] {
  const shipPositions: number[] = []

  if (isVertical) {
    for (let i = row; i < row + size; i++) {
      shipPositions.push(i * Math.sqrt(board.length) + col)
      occupyCells(board, i, col)
    }
  } else {
    for (let i = col; i < col + size; i++) {
      shipPositions.push(row * Math.sqrt(board.length) + i)
      occupyCells(board, row, i)
    }
  }

  return shipPositions
}

function occupyCells(board: Entity[], row: number, col: number): void {
  for (let i = row - 1; i <= row + 1; i++) {
    for (let j = col - 1; j <= col + 1; j++) {
      if (
        i >= 0 &&
        i < Math.sqrt(board.length) &&
        j >= 0 &&
        j < Math.sqrt(board.length)
      ) {
        board[i * Math.sqrt(board.length) + j] = 'Ship'
      }
    }
  }
}

function generateShipsField(rows: number, cols: number): number[][] {
  const board: Entity[] = Array.from({ length: rows * cols }, () => 'Empty')
  const shipLengths = [3, 2, 2, 1]
  const ships: number[][] = []

  const availableIndices = Array.from(
    { length: rows * cols },
    (_, index) => index
  )

  for (const size of shipLengths) {
    let shipPlaced = false

    for (let attempts = 0; attempts < 100; attempts++) {
      const randomIndex = Math.floor(Math.random() * availableIndices.length)
      const position = availableIndices[randomIndex]
      const isVertical = Math.random() < 0.5

      if (
        checkEmptyField(
          board,
          size,
          Math.floor(position / cols),
          position % cols,
          isVertical
        )
      ) {
        ships.push(
          placeShip(
            board,
            size,
            Math.floor(position / cols),
            position % cols,
            isVertical
          )
        )
        shipPlaced = true
        break
      }
    }

    if (!shipPlaced) {
      return generateShipsField(rows, cols)
    }
  }
  
  return ships
}

export { generateShipsField }
