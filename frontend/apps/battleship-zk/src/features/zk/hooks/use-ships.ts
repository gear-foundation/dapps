import { useAccount } from '@gear-js/react-hooks';
import { getParsedZkData, setZkData } from '../utils';

type Player = 'player' | 'enemy';

export function useShips() {
  const { account } = useAccount();

  const setPlayerShips = (ships: number[][]) => {
    if (!account?.address) {
      return;
    }

    const zkData = getParsedZkData(account);

    const newZkData = zkData
      ? { ...zkData, single: { ...zkData.single, [`ships-player`]: ships } }
      : {
          single: {
            [`ships-player`]: ships,
          },
        };

    setZkData(account, newZkData);
  };

  const getPlayerShips = () => {
    if (!account?.address) {
      return;
    }

    return getParsedZkData(account)?.single?.[`ships-player`] || null;
  };

  const setBoard = (type: Player, board: string[]) => {
    if (!account?.address) {
      return;
    }

    const zkData = getParsedZkData(account);

    const newZkData = zkData
      ? { ...zkData, single: { ...zkData.single, [`board-${type}`]: board } }
      : {
          single: {
            [`board-${type}`]: board,
          },
        };

    setZkData(account, newZkData);
  };

  const getBoard = (type: Player) => {
    if (!account?.address) {
      return;
    }

    return getParsedZkData(account)?.single?.[`board-${type}`] || null;
  };

  return { setPlayerShips, getPlayerShips, setBoard, getBoard };
}
