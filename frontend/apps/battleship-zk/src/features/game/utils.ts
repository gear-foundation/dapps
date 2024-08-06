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

export const defineDeadShip = (i: number, board: string[]) => {
  const numCols = 5;
  const markedShips: MarkedShips = {};

  const defineDeadShip = (i: number, board: string[]): string[] => {
    markedShips[i] = 1;

    if (board[i + 1] === 'BoomShip' && !markedShips[i + 1] && (i + 1) % numCols !== 0) {
      defineDeadShip(i + 1, board);
    }

    if (board[i - 1] === 'BoomShip' && !markedShips[i - 1] && (i % numCols !== 0 || i === 0)) {
      defineDeadShip(i - 1, board);
    }

    if (board[i + numCols] === 'BoomShip' && !markedShips[i + numCols]) {
      defineDeadShip(i + numCols, board);
    }

    if (board[i - numCols] === 'BoomShip' && !markedShips[i - numCols]) {
      defineDeadShip(i - numCols, board);
    }

    board[i] = 'DeadShip';

    //borders
    if (board[i + 1] === 'Unknown' && (i + 1) % numCols !== 0) {
      board[i + 1] = 'Boom';
    }
    if (board[i - 1] === 'Unknown' && i % numCols !== 0 && i > 0) {
      board[i - 1] = 'Boom';
    }
    if (board[i + numCols] === 'Unknown') {
      board[i + numCols] = 'Boom';
    }
    if (board[i - numCols] === 'Unknown') {
      board[i - numCols] = 'Boom';
    }

    //corners
    if (board[i + 1 + numCols] === 'Unknown' && (i + 1) % numCols !== 0 && (i + 1 + numCols) % numCols !== 0) {
      board[i + 1 + numCols] = 'Boom';
    }
    if (board[i + 1 - numCols] === 'Unknown' && (i + 1) % numCols !== 0 && (i + 1 - numCols) % numCols !== 0) {
      board[i + 1 - numCols] = 'Boom';
    }
    if (board[i - 1 + numCols] === 'Unknown' && i % numCols !== 0 && (i + numCols) % numCols !== 0 && i > 0) {
      board[i - 1 + numCols] = 'Boom';
    }
    if (board[i - 1 - numCols] === 'Unknown' && i % numCols !== 0 && (i - numCols) % numCols !== 0 && i > 0) {
      board[i - 1 - numCols] = 'Boom';
    }

    return board;
  };

  defineDeadShip(i, board);

  return board;
};

export function checkDeadShip(index: number, board: string[]): boolean {
  const boardSize = 5;

  if (board[index] !== 'BoomShip') {
    return false;
  }

  const visited = new Set<number>();

  const checkNeighbors = (index: number): boolean => {
    if (visited.has(index)) {
      return true;
    }

    visited.add(index);

    const row = Math.floor(index / boardSize);
    const col = index % boardSize;

    const directions = [
      [-1, 0], // up,
      [1, 0], // down
      [0, -1], // left
      [0, 1], // right
    ];

    for (let [dx, dy] of directions) {
      const newRow = row + dx;
      const newCol = col + dy;
      if (newRow >= 0 && newRow < boardSize && newCol >= 0 && newCol < boardSize) {
        const newIndex = newRow * boardSize + newCol;
        if (board[newIndex] === 'Ship') {
          return false;
        }
        if (board[newIndex] === 'BoomShip' && !checkNeighbors(newIndex)) {
          return false;
        }
      }
    }
    return true;
  };

  return checkNeighbors(index);
}
