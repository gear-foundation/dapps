import { HexString } from '@gear-js/api';
import { useAccount } from '@gear-js/react-hooks';
import clsx from 'clsx';

import { CrossIcon } from '@/assets/images';
import { Button, GameCard, Modal } from '@/components';
import { Card, RevealedPlayer } from '@/features/zk/api/types';

import { PlayerSlot } from '../../hooks';
import { getWinnersHand } from '../../utils';

import styles from './game-end-modal.module.scss';

type GameEndData = {
  pots: [string | number | bigint, `0x${string}`[]][];
  revealedPlayers: RevealedPlayer[];
  commonCardsFields: (Card | null)[];
  participants: [`0x${string}`, Participant][];
  playerSlots: PlayerSlot[];
  totalPot: number;
};

type Props = GameEndData & {
  onClose?: () => void;
};

const GameEndModal = ({ pots, revealedPlayers, commonCardsFields, participants, onClose }: Props) => {
  const { account } = useAccount();
  const myAddress = account?.decodedAddress;

  if (!myAddress || !pots?.length) return null;

  const myWinningPots = pots.filter(([_, winners]) => winners.includes(myAddress));
  const isMainPotWinner = myWinningPots.some((pot) => pot === pots[0]);

  const myTotalWinnings = myWinningPots.reduce((sum, pot) => sum + Number(pot[0]), 0);

  const getPlayerName = (address: HexString) => participants.find(([addr]) => addr === address)?.[1].name || 'Unknown';

  const getWinnersNames = (winners: HexString[]) => {
    const winnersNames = winners.map((winner) => getPlayerName(winner));

    return winnersNames.join(winnersNames.length === 2 ? ' and ' : ', ');
  };

  const isOneSidePot = pots.length === 2;

  const renderPotInfo = () => {
    if (myWinningPots.length === 0) {
      const mainPotWinners = pots[0][1];
      // ! TODO: check it. Has bug.
      const { winnersHand, handRank } = getWinnersHand(mainPotWinners, revealedPlayers, commonCardsFields) || {};

      return (
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
              {getWinnersNames(mainPotWinners)} wins main pot {Boolean(handRank) && 'with'}
            </div>
            {handRank}
          </div>
          {pots.slice(1).map(([_, winners], index) => (
            <div key={index} className={styles.sidePotInfo}>
              Side pot {isOneSidePot ? '' : index + 1} won by: {getWinnersNames(winners)}
            </div>
          ))}
        </div>
      );
    }

    // I won at least one pot
    const { winnersHand, handRank } = getWinnersHand([myAddress], revealedPlayers, commonCardsFields) || {};
    const totalPotText = `+${myTotalWinnings}`;

    const getVictoryText = () => {
      if (myWinningPots.length === 1) {
        return 'Victory';
      }
      if (isMainPotWinner) {
        return 'Main Pot Victory';
      }
      return 'Side Pot Victory';
    };

    return (
      <div>
        <h1 className={isMainPotWinner ? styles.victory : styles.sidePotWin}>{getVictoryText()}</h1>
        {winnersHand && (
          <>
            <div className={styles.winnersHand}>
              {winnersHand.map((card, index) => (
                <GameCard key={index} value={card} size="sm" />
              ))}
            </div>
            <div className={styles.handRank}>{handRank}</div>
          </>
        )}

        {/* Show my winning pots */}
        {myWinningPots.length > 1 &&
          myWinningPots.map((pot, index) => {
            const isMainPot = pot === pots[0];
            const amount = Number(pot[0]);
            return (
              <div key={index} className={styles.potAmount}>
                {isMainPot ? 'Main Pot' : `Side Pot${isOneSidePot ? '' : ` ${pots.indexOf(pot)}`}`}: +{amount}
              </div>
            );
          })}
        {/* Show other pot winners */}
        {pots.length > 1 &&
          pots
            .filter((pot) => !pot[1].includes(myAddress))
            .map((pot, index) => (
              <div key={index} className={styles.otherPotInfo}>
                {pot === pots[0] ? 'Main pot' : `Side pot ${isOneSidePot ? '' : pots.indexOf(pot)}`} won by:{' '}
                {getWinnersNames(pot[1])}
              </div>
            ))}

        <div className={styles.totalWinnings} data-text={totalPotText}>
          {totalPotText}
        </div>
      </div>
    );
  };

  return (
    <Modal
      heading=""
      className={{ modal: styles.modal, wrapper: clsx(styles.wrapper, myWinningPots.length > 0 && styles.win) }}
      onClose={onClose}>
      {renderPotInfo()}
      {onClose && (
        <Button color="grey" rounded size="x-small" onClick={onClose} className={styles.close}>
          <CrossIcon />
        </Button>
      )}
    </Modal>
  );
};

export { GameEndModal };
export type { GameEndData };
