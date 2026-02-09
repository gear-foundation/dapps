import { useNavigate } from 'react-router-dom';

import { ROUTES } from '@/app/consts';
import { Button, Modal } from '@/components';

import { useEarnedPointsQuery, useParticipantsQuery } from '../../sails';

import styles from './lobby-time-finished-modal.module.scss';

type Props = {
  isAdmin: boolean;
  isLoading: boolean;
  onCloseLobby: () => void;
};

const LobbyTimeFinishedModal = ({ isAdmin, isLoading, onCloseLobby }: Props) => {
  const navigate = useNavigate();
  const { earnedPoints } = useEarnedPointsQuery();
  const { participants } = useParticipantsQuery();

  const leaderboard =
    earnedPoints && participants
      ? [...earnedPoints]
          .map(([address, amount]) => {
            const participant = participants.find(([addr]) => addr === address)?.[1];

            return {
              address,
              name: participant?.name || 'Unknown',
              points: Number(amount),
            };
          })
          .sort((a, b) => b.points - a.points)
      : [];

  return (
    <Modal heading="Lobby time is over" className={{ wrapper: styles.wrapper }} isDark>
      {leaderboard.length > 0 && (
        <div className={styles.leaderboard}>
          <div className={styles.heading}>Leaderboard</div>
          {leaderboard.map(({ address, name, points }, index) => (
            <div key={address} className={styles.row}>
              <span className={styles.place}>{index + 1}.</span>
              <span className={styles.name}>{name}</span>
              <span className={styles.points}>{points}</span>
            </div>
          ))}
        </div>
      )}

      <div className={styles.buttons}>
        {isAdmin && (
          <Button color="danger" onClick={onCloseLobby} disabled={isLoading}>
            Close lobby
          </Button>
        )}
        <Button color="contrast" onClick={() => navigate(ROUTES.HOME)} disabled={isLoading}>
          Leave game
        </Button>
      </div>
    </Modal>
  );
};

export { LobbyTimeFinishedModal };
