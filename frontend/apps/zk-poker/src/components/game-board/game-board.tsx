import clsx from 'clsx';
import { Fragment } from 'react';

import { GameCard, PlayerSlot, PlayerCards } from '@/components';
import { Card, PlayerStatus } from '@/types';

import styles from './game-board.module.scss';
import { getCommonCardsMarginTop, getPositionSide, getSlotPositions } from './helpers';

type PlayerSlot = {
  avatar: string;
  name: string;
  status: PlayerStatus;
  chips: number;
  cards: [Card, Card] | null;
  isDiller?: boolean;
};

type Props = {
  totalPot: number;
  commonCardsFields: Card[];
  playerCards?: [Card, Card];
  playerSlots: PlayerSlot[];
};

const GameBoard = ({ totalPot, commonCardsFields, playerCards, playerSlots }: Props) => {
  const isMovedTotalPot = playerSlots.length === 1 || playerSlots.length === 2;

  const commonCardsMarginTop = getCommonCardsMarginTop(playerSlots.length);
  const slotPositions = getSlotPositions(playerSlots.length);

  return (
    <div className={styles.boardWrapper}>
      <div className={styles.board}>
        <div className={styles.boardBorderMiddle}>
          <div className={styles.boardBorderInner}>
            <div className={styles.boardField}>
              <div className={styles.boardFieldLine1}>
                <div className={styles.boardFieldLine2}>
                  <div className={styles.commonCards} style={{ marginTop: commonCardsMarginTop }}>
                    <div className={clsx(styles.totalPot, { [styles.low]: isMovedTotalPot })}>
                      <div className={styles.totalPotValue}>{totalPot}</div>
                      <div className={styles.totalPotText}>Total pot</div>
                    </div>

                    {commonCardsFields.map((card, index) => (
                      <GameCard value={card} size="md" isDashed key={index} />
                    ))}

                    {playerCards && (
                      <div className={styles.playerCards}>
                        {playerCards.map((card, index) => (
                          <GameCard value={card} size="lg" key={index} />
                        ))}
                      </div>
                    )}
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
        <div className={styles.playerSlots}>
          {playerSlots.map(({ isDiller, cards, ...playerSlot }, index) => {
            const side = getPositionSide(playerSlots.length, index);
            const [playerTopOffset, cardsTopOffset] = slotPositions[index];

            return (
              <Fragment key={`${playerSlot.name}-${index}`}>
                <PlayerSlot {...playerSlot} top={playerTopOffset} side={side} />
                <PlayerCards isDiller={isDiller} cards={cards} top={cardsTopOffset} side={side} />
              </Fragment>
            );
          })}
        </div>
      </div>
    </div>
  );
};

export { GameBoard };
