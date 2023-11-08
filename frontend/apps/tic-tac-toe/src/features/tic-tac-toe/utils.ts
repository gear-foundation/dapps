import { Cell } from './types';
import { toNumber } from '@/app/utils';

export function calculateWinner(squares: Cell[]) {
  const lines: [number[], string][] = [
    [[0, 1, 2], 'row-1'],
    [[3, 4, 5], 'row-2'],
    [[6, 7, 8], 'row-3'],
    [[0, 3, 6], 'col-1'],
    [[1, 4, 7], 'col-2'],
    [[2, 5, 8], 'col-3'],
    [[0, 4, 8], 'diagonal-1'],
    [[2, 4, 6], 'diagonal-2'],
  ];
  for (let i = 0; i < lines.length; i++) {
    const [a, b, c] = lines[i][0];
    if (squares[a] && squares[a] === squares[b] && squares[a] === squares[c]) {
      return lines[i];
    }
  }
}

export function calculateWinRate(wins: string, games: string) {
  return Math.floor((toNumber(wins) / toNumber(games) || 0) * 10000) / 100;
}
