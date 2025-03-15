import { useAccount, useBalanceFormat } from '@gear-js/react-hooks';
import clsx from 'clsx';
import { useAtomValue } from 'jotai';

import { Player, State } from '@/app/utils';
import { Card, Text } from '@/components';
import { VaraIcon } from '@/components/layout';

import { battleHistoryAtom, currentPlayersAtom } from '../../store';

import styles from './game-over-card.module.scss';

type GameOverCardProps = {
  bid: number;
  totalParticipants: number;
  state: State;
  participantsMap: Record<string, Player>;
  isAlive: boolean;
  isSpectating: boolean;
  onScrollToHistoryClick: () => void;
  className?: string;
};

const STATUS = {
  WIN: 'win',
  LOSS: 'loss',
  DRAW: 'draw',
} as const;

const STATUS_TEXT = {
  [STATUS.WIN]: 'You won',
  [STATUS.LOSS]: 'You lost',
  [STATUS.DRAW]: "It's a draw",
} as const;

const GameOverCard = ({
  bid,
  className,
  state,
  totalParticipants,
  participantsMap,
  isAlive,
  isSpectating,
  onScrollToHistoryClick,
}: GameOverCardProps) => {
  const { account } = useAccount();
  const currentPlayers = useAtomValue(currentPlayersAtom);

  const { getFormattedBalanceValue } = useBalanceFormat();
  const prizeValue = getFormattedBalanceValue(bid).toNumber() * totalParticipants;

  const isTournamentOver = 'gameIsOver' in state;
  const { winners } = isTournamentOver ? state.gameIsOver : {};
  const [firstWinner, secondWinner] = winners || [undefined, undefined];
  const isTournamentDraw = isTournamentOver && Boolean(secondWinner);

  const battleHistory = useAtomValue(battleHistoryAtom);
  const [lastTurn] = battleHistory || [undefined];
  const isBattleDraw = !isTournamentOver && lastTurn?.player.health === 0 && lastTurn?.opponent.health === 0;

  const getStatus = () => {
    if (!account) return;

    if (isBattleDraw || (isTournamentDraw && winners?.includes(account.decodedAddress))) return STATUS.DRAW;

    if (!isAlive && (!isSpectating || isTournamentOver)) return STATUS.LOSS;

    if (isTournamentOver && firstWinner === account.decodedAddress) return STATUS.WIN;
  };

  const status = getStatus();

  const getDescription = () => {
    if (!isTournamentOver) {
      const { player, opponent } = currentPlayers || {};
      const winnerName = player?.player_settings.health === 0 ? opponent?.user_name : player?.user_name;

      return `${winnerName || 'Player 2'} wins! Now you can watch other players' battles.`;
    }

    const firstWinnerName = participantsMap[firstWinner!]?.user_name;
    const secondWinnerName = participantsMap[secondWinner!]?.user_name;

    if (isTournamentDraw) return `${firstWinnerName} and ${secondWinnerName} ended in a draw!`;
    return `${firstWinnerName} wins!`;
  };

  if (!status) return;

  return (
    <div className={clsx(styles.backdrop, status === STATUS.LOSS && styles.grayedOut, className)}>
      {!isBattleDraw && (
        <Card title={STATUS_TEXT[status]} description={getDescription()} className={styles.card} size="md">
          {isTournamentOver ? (
            <div className={styles.prize}>
              <Text size="sm">Winner prize:</Text>
              <VaraIcon className={styles.icon} />
              <Text size="sm" weight="semibold">
                {isTournamentDraw ? prizeValue / 2 : prizeValue} VARA
              </Text>
            </div>
          ) : (
            <button type="button" className={styles.scrollToHistoryButton} onClick={onScrollToHistoryClick}>
              Choose any battle from the list below
            </button>
          )}
        </Card>
      )}
    </div>
  );
};

export { GameOverCard };
