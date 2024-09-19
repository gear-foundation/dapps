import { useAccount } from '@gear-js/react-hooks';
import { getParsedZkData, setZkData } from '../utils';
import { checkDeadShip, defineDeadShip } from '@/features/game/utils';
import { isNull, isUndefined } from '@polkadot/util';

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

    board[player_step] = stepResult;
    setBoard(gameType, 'enemy', board);
  };

  const checkIsStepOnShip = (gameType: GameType, enemyStep?: number | null, playerType: PlayerType = 'player') => {
    const board = getBoard(gameType, playerType);

    if (!board || isNull(enemyStep) || isUndefined(enemyStep)) {
      return;
    }
    const isDeadShip = checkDeadShip(enemyStep, board);

    return {
      isBoomShip: board[enemyStep] === 'BoomShip',
      isDeadShip,
    };
  };

  const updatePlayerBoard = (gameType: GameType, enemy_step: number) => {
    let board = getBoard(gameType, 'player');

    if (!board) {
      return;
    }

    if (board[enemy_step] === 'Empty') {
      board[enemy_step] = 'Boom';
    }

    if (board[enemy_step] === 'Ship') {
      board[enemy_step] = 'BoomShip';
      if (checkDeadShip(enemy_step, board)) {
        board = defineDeadShip(enemy_step, board);
      }
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
    checkIsStepOnShip,
    updateEnemyBoard,
    updatePlayerBoard,
  };
}
