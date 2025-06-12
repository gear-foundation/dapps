import clsx from 'clsx';

import { GameCard, Modal } from '@/components';
import { Card, HandRank } from '@/features/zk/api/types';

import styles from './game-end-modal.module.scss';

type Props = {
  winnerName: string;
  pot: number;
  winnersHand?: Card[];
  handRank?: HandRank;
  isWinner: boolean;
};

const GameEndModal = ({ winnerName, pot, winnersHand, handRank, isWinner }: Props) => {
  // ! TODO: remove it when fixed
  if (!winnersHand || !handRank)
    return (
      <Modal heading="" className={{ modal: styles.modal, wrapper: clsx(styles.wrapper, isWinner && styles.win) }}>
        <div>
          <h1 className={styles.lose}>No one wins</h1>
        </div>
      </Modal>
    );

  const rank = handRank.replace('-', ' ');
  const potText = `+${pot}`;

  return (
    <Modal heading="" className={{ modal: styles.modal, wrapper: clsx(styles.wrapper, isWinner && styles.win) }}>
      {isWinner ? (
        <div>
          <h1 className={styles.victory}>Victory</h1>
          <div className={styles.winnersHand}>
            {winnersHand.map((card, index) => (
              <GameCard key={index} value={card} size="sm" />
            ))}
          </div>
          <div className={styles.handRank}>{rank}</div>
          <div className={styles.pot} data-text={potText}>
            {potText}
          </div>
        </div>
      ) : (
        <div>
          <h1 className={styles.lose}>You lose</h1>
          <div className={styles.winnersHand}>
            {winnersHand.map((card, index) => (
              <GameCard key={index} value={card} size="sm" />
            ))}
          </div>

          <div className={clsx(styles.handRank, styles.winner)}>
            <div>{winnerName} wins with</div>
            {rank}
          </div>
        </div>
      )}
    </Modal>
  );
};

export { GameEndModal };
