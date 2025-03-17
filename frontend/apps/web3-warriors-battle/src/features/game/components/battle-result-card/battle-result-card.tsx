import clsx from 'clsx';
import { useAtomValue } from 'jotai';

import { useMyBattleQuery } from '@/app/utils';
import { Card } from '@/components';

import { currentPlayersAtom } from '../../store';

import styles from './battle-result-card.module.scss';

type GameOverCardProps = {
  isTournamentOver: boolean;
  isAlive: boolean;
  isSpectating: boolean;
  onScrollToHistoryClick: () => void;
};

const BattleResultCard = ({ isTournamentOver, isAlive, isSpectating, onScrollToHistoryClick }: GameOverCardProps) => {
  const { battleState } = useMyBattleQuery();
  const currentPlayers = useAtomValue(currentPlayersAtom);

  if (!battleState || !currentPlayers) return;

  const { pairs } = battleState;
  const isAnyActiveBattle = pairs.some(([, { player_2 }]) => !player_2.startsWith('0x00'));

  const { player, opponent } = currentPlayers;
  const playerStats = player.player_settings;
  const opponentStats = opponent.player_settings;
  const isBattleOver = playerStats.health === 0 || opponentStats.health === 0;
  const isDraw = playerStats.health === 0 && opponentStats.health === 0;
  const winnerName = playerStats.health === 0 ? opponent.user_name : player.user_name;

  if (!isBattleOver || (isAlive && !isTournamentOver)) return;

  const getTitle = () => {
    if (isDraw) return "It's a draw!";
    if (isSpectating) return `${winnerName} wins!`;

    return isAlive ? 'You win' : 'You lose';
  };

  const getDescription = () => {
    const drawText = `${player.user_name} and ${opponent.user_name} ended in a draw!`;

    if (isSpectating) {
      if (!isTournamentOver)
        return isAnyActiveBattle
          ? 'You can wait for the new battle here or choose another one from the battles list.'
          : "As soon as the remaining players start their battles, you'll be able to watch them.";

      return isDraw ? drawText : '';
    }

    if (isDraw) return drawText;

    if (!isTournamentOver && !isAlive)
      return isAnyActiveBattle
        ? `${winnerName} wins! Now you can watch other players' battles.`
        : `${winnerName} wins! As soon as the remaining players start their battles, you'll be able to watch them.`;

    return `${winnerName} wins!`;
  };

  return (
    <div className={clsx(styles.backdrop, !isSpectating && !isAlive && styles.grayedOut)}>
      <Card title={getTitle()} description={getDescription()} className={styles.card} size="md">
        {isAnyActiveBattle && !isTournamentOver && !isSpectating && !isAlive && (
          <button type="button" className={styles.scrollToHistoryButton} onClick={onScrollToHistoryClick}>
            Choose any battle from the list below
          </button>
        )}
      </Card>
    </div>
  );
};

export { BattleResultCard };
