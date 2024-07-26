import { useNavigate } from 'react-router-dom';
import { decodeAddress } from '@gear-js/api';
import { Button } from '@gear-js/vara-ui';
import { ModalBottom } from '@/components/ui/modal';
import { Text } from '@/components/ui/text';
import { clearZkData } from '@/features/zk/utils';
import { useAccount } from '@gear-js/react-hooks';
import { GameType } from '@/features/game/types';
import { ROUTES } from '@/app/consts';
import styles from './GameEndModal.module.scss';

type Props = {
  onClose: () => void;
  resetGameState: () => void;
  time: string;
  totalShoots: number;
  successfulShoots: number;
  efficiency: string | number;
  winner: string;
  gameType: GameType;
};

export default function GameEndModal({
  onClose,
  resetGameState,
  time,
  totalShoots,
  successfulShoots,
  efficiency,
  winner,
  gameType,
}: Props) {
  const { account } = useAccount();
  const navigate = useNavigate();

  const defineGameResults = () => {
    if (gameType === 'single') {
      return winner === 'Player' ? 'You win' : 'You Lose';
    }

    return decodeAddress(winner) === account?.decodedAddress ? 'You win' : 'You Lose';
  };

  const clearLocalData = () => {
    if (account?.address) {
      clearZkData('single', account);
    }
  };

  const handleExit = () => {
    navigate(ROUTES.HOME);
    clearLocalData();
    resetGameState();
  };

  const handlePlayAgain = () => {
    clearLocalData();
    resetGameState();
  };

  return (
    <ModalBottom heading={defineGameResults()} onClose={onClose}>
      <div className={styles.content}>
        <Text>Awesome! Play again to improve your skills.</Text>

        <div className={styles.gameInfo}>
          <div className={styles.line}>
            <Text>Time:</Text>
            <hr />
            <Text weight="semibold">{time}</Text>
          </div>
          <div className={styles.line}>
            <Text>Total shots:</Text>
            <hr />
            <Text weight="semibold">{totalShoots}</Text>
          </div>
          <div className={styles.line}>
            <Text>Successful hits:</Text>
            <hr />
            <Text weight="semibold">{successfulShoots}</Text>
          </div>
          <div className={styles.line}>
            <Text>Efficiency:</Text>
            <hr />
            <Text weight="semibold">{efficiency}%</Text>
          </div>
        </div>
        <div className={styles.buttons}>
          <Button color="dark" text="Exit" onClick={handleExit} />
          {gameType === 'single' && <Button color="primary" text="Play again" onClick={handlePlayAgain} />}
        </div>
      </div>
    </ModalBottom>
  );
}
