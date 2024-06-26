import { useAccount } from '@gear-js/react-hooks';
import { getParsedZkData, setZkData } from '../utils';
import { defineDeadShip } from '@/features/game/utils';
import { StepResult } from '@/features/game/assets/lib/lib';

type PlayerType = 'player' | 'enemy';
type GameType = 'single' | 'multi';

export function useShips() {
  const { account } = useAccount();

  const setPlayerShips = (gameType: GameType, ships: number[][]) => {
    if (!account?.address) {
      return;
    }

    const zkData = getParsedZkData(account);

    const newZkData = zkData
      ? { ...zkData, [gameType]: { ...zkData[gameType], [`ships-player`]: ships } }
      : {
          [gameType]: {
            [`ships-player`]: ships,
          },
        };

    setZkData(account, newZkData);
  };

  const getPlayerShips = (gameType: GameType) => {
    if (!account?.address) {
      return;
    }

    return getParsedZkData(account)?.[gameType]?.[`ships-player`] || null;
  };

  const setBoard = (gameType: GameType, playerType: PlayerType, board: string[]) => {
    if (!account?.address) {
      return;
    }

    const zkData = getParsedZkData(account);

    const newZkData = zkData
      ? { ...zkData, [gameType]: { ...zkData[gameType], [`board-${playerType}`]: board } }
      : {
          [gameType]: {
            [`board-${playerType}`]: board,
          },
        };

    setZkData(account, newZkData);
  };

  const getBoard = (gameType: GameType, playerType: PlayerType) => {
    if (!account?.address) {
      return;
    }

    return getParsedZkData(account)?.[gameType]?.[`board-${playerType}`] || null;
  };

  const setPlayerHits = (gameType: GameType, hits: number[]) => {
    if (!account?.address) {
      return;
    }

    const zkData = getParsedZkData(account);

    const newZkData = zkData
      ? { ...zkData, [gameType]: { ...zkData[gameType], [`hits-player`]: hits } }
      : {
          [gameType]: {
            [`hits-player`]: hits,
          },
        };

    setZkData(account, newZkData);
  };

  const getPlayerHits = (gameType: GameType) => {
    if (!account?.address) {
      return;
    }

    return getParsedZkData(account)?.[gameType]?.['hits-player'] || null;
  };

  const createPlayerHits = (gameType: GameType) => {
    setPlayerHits(gameType, Array.from(Array(25).keys()).fill(-1));
  };

  const updatePlayerHits = (gameType: GameType, hit: number) => {
    const hits = getPlayerHits(gameType);

    if (!hits) {
      throw new Error('No hits saved!');
    }

    if (hits.includes(hit)) {
      return;
    }

    const firstEmptyIndex = hits.findIndex((item) => item === -1);

    setPlayerHits(gameType, [...hits.slice(0, firstEmptyIndex), hit, ...hits.slice(firstEmptyIndex + 1)]);
  };

  const updateEnemyBoard = (gameType: GameType, stepResult: string, player_step: number) => {
    const board = getBoard(gameType, 'enemy');

    if (!board) {
      return;
    }

    if (stepResult === 'DeadShip') {
      const updatedBoard = defineDeadShip(player_step, board);

      setBoard(gameType, 'enemy', updatedBoard);
      return;
    }

    board[player_step] = stepResult as string;
    setBoard(gameType, 'enemy', board);
  };

  const updatePlayerBoard = (gameType: GameType, bot_step: number) => {
    const board = getBoard(gameType, 'player');

    if (!board) {
      return;
    }

    if (board[bot_step] === 'Empty') {
      board[bot_step] = 'Boom';
    }

    if (board[bot_step] === 'Ship') {
      board[bot_step] = 'BoomShip';
    }

    setBoard(gameType, 'player', board);
  };

  return {
    updatePlayerHits,
    getPlayerHits,
    setPlayerShips,
    getPlayerShips,
    setBoard,
    getBoard,
    createPlayerHits,
    updateEnemyBoard,
    updatePlayerBoard,
  };
}
