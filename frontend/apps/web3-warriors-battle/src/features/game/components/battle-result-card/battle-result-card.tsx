import { useBalanceFormat } from '@gear-js/react-hooks';
import clsx from 'clsx';
import { useAtomValue } from 'jotai';

import { Card, Text } from '@/components';
import { VaraIcon } from '@/components/layout';

import { currentPlayersAtom } from '../../store';

import styles from './battle-result-card.module.scss';

type GameOverCardProps = {
  bid: number;
  totalParticipants: number;
  isTournamentOver: boolean;
  isAlive: boolean;
  isSpectating: boolean;
  onScrollToHistoryClick: () => void;
};

const BattleResultCard = ({
  bid,
  isTournamentOver,
  totalParticipants,
  isAlive,
  isSpectating,
  onScrollToHistoryClick,
}: GameOverCardProps) => {
  const currentPlayers = useAtomValue(currentPlayersAtom);

  const { getFormattedBalanceValue } = useBalanceFormat();
  const prizeValue = getFormattedBalanceValue(bid).toNumber() * totalParticipants;

  if (!currentPlayers) return;

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
    if (isSpectating) {
      if (!isTournamentOver) return 'You can wait for the new battle here or choose another one from the battles list.';

      return isDraw ? `${player.user_name} and ${opponent.user_name} ended in a draw!` : ``;
    } else {
      if (isDraw) return `${player.user_name} and ${opponent.user_name} ended in a draw!`;
      if (!isTournamentOver && !isAlive) return `${winnerName} wins! Now you can watch other players' battles.`;

      return `${winnerName} wins!`;
    }
  };

  return (
    <div className={clsx(styles.backdrop, !isSpectating && !isAlive && styles.grayedOut)}>
      <Card title={getTitle()} description={getDescription()} className={styles.card} size="md">
        {isTournamentOver ? (
          <div className={styles.prize}>
            <Text size="sm">Winner prize:</Text>
            <VaraIcon className={styles.icon} />
            <Text size="sm" weight="semibold">
              {isDraw ? prizeValue / 2 : prizeValue} VARA
            </Text>
          </div>
        ) : (
          !isSpectating &&
          !isAlive && (
            <button type="button" className={styles.scrollToHistoryButton} onClick={onScrollToHistoryClick}>
              Choose any battle from the list below
            </button>
          )
        )}
      </Card>
    </div>
  );
};

export { BattleResultCard };
