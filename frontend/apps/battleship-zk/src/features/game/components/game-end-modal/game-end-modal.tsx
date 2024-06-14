import { ModalBottom } from '@/components/ui/modal';
import { Button } from '@gear-js/vara-ui';
import { Text } from '@/components/ui/text';

import styles from './GameEndModal.module.scss';
import { useGame } from '../../hooks';
import { useNavigate } from 'react-router-dom';
import { clearZkData } from '@/features/zk/utils';
import { useAccount } from '@gear-js/react-hooks';

type BattleshipParticipants = 'Player' | 'Bot';

type Props = {
  onClose: () => void;
  time: string;
  totalShoots: number;
  successfulShoots: number;
  efficiency: string | number;
  gameResult: BattleshipParticipants | null;
};

export default function GameEndModal({ onClose, time, totalShoots, successfulShoots, efficiency, gameResult }: Props) {
  const { account } = useAccount();
  const navigate = useNavigate();
  const { resetGameState } = useGame();

  const heading = gameResult === 'Player' ? 'You win' : 'You Lose';

  const clearLocalData = () => {
    if (account?.address) {
      clearZkData(account);
    }
  };

  const handleExit = () => {
    clearLocalData();
    navigate('/');
  };

  const handlePlayAgain = () => {
    clearLocalData();
    resetGameState();
  };

  return (
    <ModalBottom heading={heading} onClose={onClose}>
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
          <Button color="primary" text="Play again" onClick={handlePlayAgain} />
        </div>
      </div>
    </ModalBottom>
  );
}
