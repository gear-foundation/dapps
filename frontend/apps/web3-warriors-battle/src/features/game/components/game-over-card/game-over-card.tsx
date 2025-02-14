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
  isShowOtherBattle: boolean;
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
  isShowOtherBattle,
}: GameOverCardProps) => {
  const { account } = useAccount();
  const currentPlayers = useAtomValue(currentPlayersAtom);
  const battleHistory = useAtomValue(battleHistoryAtom);
  const { getFormattedBalanceValue } = useBalanceFormat();
  const isTournamentOver = 'gameIsOver' in state;
  const prizeValue = Number(getFormattedBalanceValue(Number(bid) || 0)) * totalParticipants;
  const isCurrentDraw =
    !isTournamentOver && battleHistory?.[0].player.health === 0 && battleHistory?.[0].opponent.health === 0;

  const isTournamentDraw = isTournamentOver && state.gameIsOver.winners[1];

  const getStatus = () => {
    if (!account) return;

    if (isCurrentDraw || (isTournamentDraw && state.gameIsOver.winners.includes(account.decodedAddress)))
      return STATUS.DRAW;

    if (!isAlive && (!isShowOtherBattle || isTournamentOver)) return STATUS.LOSS;

    if (isTournamentOver && state.gameIsOver.winners[0] === account.decodedAddress) return STATUS.WIN;
  };

  const status = getStatus();

  const getDesctiptionText = () => {
    if (!isTournamentOver) {
      const winnersName =
        currentPlayers?.player.player_settings.health === 0
          ? currentPlayers?.opponent.user_name
          : currentPlayers?.player.user_name;
      return `${
        winnersName || 'Player 2'
      } wins! Now you can watch other players' battles. Choose any battle from the list below.`;
    }

    const firstTournamentWinnerName = participantsMap[state.gameIsOver.winners[0]].user_name;
    if (isTournamentDraw && state.gameIsOver.winners[1]) {
      const secondTournamentWinnerName = participantsMap[state.gameIsOver.winners[1]].user_name;
      return `${firstTournamentWinnerName} and ${secondTournamentWinnerName} ended in a draw!`;
    } else {
      return `${firstTournamentWinnerName} wins!`;
    }
  };

  return (
    status && (
      <div className={clsx(styles.backdrop, status === STATUS.LOSS && styles.grayedOut, className)}>
        {!isCurrentDraw && (
          <Card title={STATUS_TEXT[status]} description={getDesctiptionText()} className={styles.card} size="md">
            {isTournamentOver && (
              <div className={styles.prize}>
                <Text size="sm">Winner prize:</Text>
                <VaraIcon className={styles.icon} />
                <Text size="sm" weight="semibold">
                  {isTournamentDraw ? prizeValue / 2 : prizeValue} VARA
                </Text>
              </div>
            )}
          </Card>
        )}
      </div>
    )
  );
};

export { GameOverCard };
