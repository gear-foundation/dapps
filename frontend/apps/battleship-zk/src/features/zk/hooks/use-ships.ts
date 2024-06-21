import { useAccount } from '@gear-js/react-hooks';
import { getParsedZkData, setZkData } from '../utils';

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

  return { setPlayerShips, getPlayerShips, setBoard, getBoard };
}
