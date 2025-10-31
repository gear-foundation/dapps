type ShipCell = 'Empty' | 'Unknown' | 'Ship';

type ShipLayout = ShipCell[];

type MarkedShips = {
  [key: string]: 1 | 0;
};

export const getShipLayout = (shipStatusArray: string[]): number[][] => {
  const shipLayout: number[][] = [];

  let currentShip: number[] = [];
  shipStatusArray.forEach((status, index) => {
    if (status === 'Ship' || status === 'BoomShip') {
      currentShip.push(index);
    } else if (currentShip.length > 0) {
      shipLayout.push(currentShip);
      currentShip = [];
    }
  });

  if (currentShip.length > 0) {
    shipLayout.push(currentShip);
  }

  return shipLayout;
};

export function convertShipsToField(
  shipPositions: number[][],
  rows: number,
  cols: number,
  emptyCellName?: ShipCell,
): ShipLayout {
  const field: ShipLayout = Array.from({ length: rows * cols }, () => emptyCellName || 'Empty');

  shipPositions.forEach((ship) => {
    ship.forEach((position) => {
      field[position] = 'Ship';
    });
  });

  return field;
}

export const getFormattedTime = (time: number, isPadMinutes = true) => {
  const minutes = Math.floor(time / (1000 * 60));
  const seconds = Math.floor((time % (1000 * 60)) / 1000);
  const formattedMunutes = isPadMinutes ? String(minutes).padStart(2, '0') : minutes;

  const formattedTime = `${formattedMunutes}:${String(seconds).padStart(2, '0')}`;

  return formattedTime;
};

export const defineDeadShip = (cellIndex: number, board: string[]) => {
  const numCols = 5;
  const markedShips: MarkedShips = {};

  const markDeadShip = (position: number, currentBoard: string[]): string[] => {
    markedShips[position] = 1;

    if (currentBoard[position + 1] === 'BoomShip' && !markedShips[position + 1] && (position + 1) % numCols !== 0) {
      markDeadShip(position + 1, currentBoard);
    }

    if (
      currentBoard[position - 1] === 'BoomShip' &&
      !markedShips[position - 1] &&
      (position % numCols !== 0 || position === 0)
    ) {
      markDeadShip(position - 1, currentBoard);
    }

    if (currentBoard[position + numCols] === 'BoomShip' && !markedShips[position + numCols]) {
      markDeadShip(position + numCols, currentBoard);
    }

    if (currentBoard[position - numCols] === 'BoomShip' && !markedShips[position - numCols]) {
      markDeadShip(position - numCols, currentBoard);
    }

    currentBoard[position] = 'DeadShip';

    if (currentBoard[position + 1] === 'Unknown' && (position + 1) % numCols !== 0) {
      currentBoard[position + 1] = 'Boom';
    }
    if (currentBoard[position - 1] === 'Unknown' && position % numCols !== 0 && position > 0) {
      currentBoard[position - 1] = 'Boom';
    }
    if (currentBoard[position + numCols] === 'Unknown') {
      currentBoard[position + numCols] = 'Boom';
    }
    if (currentBoard[position - numCols] === 'Unknown') {
      currentBoard[position - numCols] = 'Boom';
    }

    if (
      currentBoard[position + 1 + numCols] === 'Unknown' &&
      (position + 1) % numCols !== 0 &&
      (position + 1 + numCols) % numCols !== 0
    ) {
      currentBoard[position + 1 + numCols] = 'Boom';
    }
    if (
      currentBoard[position + 1 - numCols] === 'Unknown' &&
      (position + 1) % numCols !== 0 &&
      (position + 1 - numCols) % numCols !== 0
    ) {
      currentBoard[position + 1 - numCols] = 'Boom';
    }
    if (
      currentBoard[position - 1 + numCols] === 'Unknown' &&
      position % numCols !== 0 &&
      (position + numCols) % numCols !== 0 &&
      position > 0
    ) {
      currentBoard[position - 1 + numCols] = 'Boom';
    }
    if (
      currentBoard[position - 1 - numCols] === 'Unknown' &&
      position % numCols !== 0 &&
      (position - numCols) % numCols !== 0 &&
      position > 0
    ) {
      currentBoard[position - 1 - numCols] = 'Boom';
    }

    return currentBoard;
  };

  markDeadShip(cellIndex, board);

  return board;
};

export function checkDeadShip(index: number, board: string[]): boolean {
  const boardSize = 5;

  if (!['BoomShip', 'DeadShip'].includes(board[index])) {
    return false;
  }

  const visited = new Set<number>();

  const traverseNeighbors = (cell: number): boolean => {
    if (visited.has(cell)) {
      return true;
    }

    visited.add(cell);

    const row = Math.floor(cell / boardSize);
    const col = cell % boardSize;

    const directions = [
      [-1, 0], // up,
      [1, 0], // down
      [0, -1], // left
      [0, 1], // right
    ];

    for (const [dx, dy] of directions) {
      const newRow = row + dx;
      const newCol = col + dy;
      if (newRow >= 0 && newRow < boardSize && newCol >= 0 && newCol < boardSize) {
        const newIndex = newRow * boardSize + newCol;
        if (board[newIndex] === 'Ship') {
          return false;
        }
        if (['BoomShip', 'DeadShip'].includes(board[newIndex]) && !traverseNeighbors(newIndex)) {
          return false;
        }
      }
    }
    return true;
  };

  return traverseNeighbors(index);
}
