import { HexString } from '@gear-js/api';
import clsx from 'clsx';
import { Fragment } from 'react';

import { GameCard, PlayerSlot, PlayerCards } from '@/components';
import { Card, PlayerStatus } from '@/features/zk/api/types';

import styles from './game-board.module.scss';
import { getCommonCardsMarginTop, getPositionSide, getSlotPositions } from './helpers';

type PlayerSlot = {
  address?: HexString;
  name: string;
  status: PlayerStatus;
  chips: number;
  cards?: [Card, Card] | null;
  isDiller?: boolean;
  isMe?: boolean;
};

type Props = {
  totalPot?: number;
  commonCardsFields: (Card | null)[];
  playerSlots: PlayerSlot[];
  timePerMoveMs: number;
};

const GameBoard = ({ totalPot, commonCardsFields, playerSlots, timePerMoveMs }: Props) => {
  const isMovedTotalPot = playerSlots.length === 1 || playerSlots.length === 2;

  const myIndex = playerSlots.findIndex((playerSlot) => playerSlot.isMe);
  const mySlot = myIndex !== -1 ? playerSlots[myIndex] : null;

  const slotsAfterMe = playerSlots.slice(myIndex + 1);
  const slotsBeforeMe = playerSlots.slice(0, myIndex);
  const reorderedSlots = mySlot ? [...slotsAfterMe, ...slotsBeforeMe] : playerSlots;

  const commonCardsMarginTop = getCommonCardsMarginTop(reorderedSlots.length);
  const slotPositions = getSlotPositions(reorderedSlots.length);

  return (
    <div className={styles.boardWrapper}>
      <div className={styles.board}>
        <div className={styles.boardBorderMiddle}>
          <div className={styles.boardBorderInner}>
            <div className={styles.boardField}>
              <div className={styles.boardFieldLine1}>
                <div className={styles.boardFieldLine2}>
                  <div className={styles.commonCards} style={{ marginTop: commonCardsMarginTop }}>
                    {totalPot && (
                      <div className={clsx(styles.totalPot, { [styles.low]: isMovedTotalPot })}>
                        <div className={styles.totalPotValue}>{totalPot}</div>
                        <div className={styles.totalPotText}>Total pot</div>
                      </div>
                    )}

                    {commonCardsFields.map((card, index) => (
                      <GameCard value={card} size="md" isDashed key={index} />
                    ))}

                    {mySlot?.cards && (
                      <div className={styles.myCards}>
                        {mySlot.cards.map((card, index) => (
                          <GameCard value={card} size="lg" key={index} />
                        ))}
                      </div>
                    )}
                    {mySlot && (
                      <PlayerSlot
                        {...mySlot}
                        side="bottom"
                        hideAvatar={!!mySlot?.cards}
                        timePerMoveMs={timePerMoveMs}
                      />
                    )}
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
        <div className={styles.playerSlots}>
          {reorderedSlots.map(({ isDiller, cards, ...playerSlot }, index) => {
            const side = getPositionSide(reorderedSlots.length, index);
            const [playerTopOffset, cardsTopOffset] = slotPositions[index];
            const hasCards = cards !== undefined;

            return (
              <Fragment key={`${playerSlot.name}-${index}`}>
                <PlayerSlot {...playerSlot} top={playerTopOffset} side={side} timePerMoveMs={timePerMoveMs} />
                {hasCards && <PlayerCards isDiller={isDiller} cards={cards} top={cardsTopOffset} side={side} />}
              </Fragment>
            );
          })}
        </div>
      </div>
    </div>
  );
};

export { GameBoard };
