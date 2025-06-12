import { useAccount } from '@gear-js/react-hooks';
import clsx from 'clsx';

import { GameCard, Modal } from '@/components';
import { Card, RevealedPlayer } from '@/features/zk/api/types';

import { getWinnersHand } from '../../utils';

import styles from './game-end-modal.module.scss';

type Props = {
  winners?: `0x${string}`[];
  cashPrize?: (string | number | bigint)[] | undefined;
  revealedPlayers?: RevealedPlayer[];
  commonCardsFields?: (Card | null)[];
  participants: [`0x${string}`, Participant][];
};

const GameEndModal = ({ winners, cashPrize, revealedPlayers, commonCardsFields, participants }: Props) => {
  const { account } = useAccount();

  const myWinnerIndex = winners?.findIndex((winner) => winner === account?.decodedAddress);
  const isWinner = myWinnerIndex !== undefined && myWinnerIndex !== -1;
  const myWinnerCashPrize = isWinner && cashPrize ? Number(cashPrize[myWinnerIndex]) : undefined;
  const winnerName = participants?.find(([address]) => winners?.includes(address))?.[1].name || '';

  const { winnersHand, handRank } = getWinnersHand(winners, revealedPlayers, commonCardsFields) || {};

  const rank = handRank?.replace('-', ' ');
  const potText = `+${myWinnerCashPrize}`;

  // ! TODO: if 2 or more winners, we need to show all winners
  return (
    <Modal heading="" className={{ modal: styles.modal, wrapper: clsx(styles.wrapper, isWinner && styles.win) }}>
      {isWinner ? (
        <div>
          <h1 className={styles.victory}>Victory</h1>
          {winnersHand && (
            <>
              <div className={styles.winnersHand}>
                {winnersHand.map((card, index) => (
                  <GameCard key={index} value={card} size="sm" />
                ))}
              </div>
              <div className={styles.handRank}>{rank}</div>
            </>
          )}
          <div className={styles.pot} data-text={potText}>
            {potText}
          </div>
        </div>
      ) : (
        <div>
          <h1 className={styles.lose}>You lose</h1>
          {winnersHand && (
            <div className={styles.winnersHand}>
              {winnersHand.map((card, index) => (
                <GameCard key={index} value={card} size="sm" />
              ))}
            </div>
          )}

          <div className={clsx(styles.handRank, styles.winner)}>
            <div>
              {winnerName} wins {!!rank && 'with'}
            </div>
            {rank}
          </div>
        </div>
      )}
    </Modal>
  );
};

export { GameEndModal };
