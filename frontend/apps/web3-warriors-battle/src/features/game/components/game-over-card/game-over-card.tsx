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

  const getMyResultStatus = () => {
    if (!account) return null;
    if (isCurrentDraw || (isTournamentDraw && state.gameIsOver.winners.includes(account.decodedAddress)))
      return 'It’s a draw';
    if (!isAlive && (!isShowOtherBattle || isTournamentOver)) return 'You lose';
    if (isTournamentOver && state.gameIsOver.winners[0] === account.decodedAddress) return 'You win';
    return null;
  };

  const myResultStatus = getMyResultStatus();

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
    myResultStatus && (
      <div className={clsx(styles.backdrop, className)}>
        {!isCurrentDraw && (
          <Card title="Game over" description={getDesctiptionText()} className={styles.card} size="md">
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

        <p className={styles.result}>{myResultStatus}</p>
      </div>
    )
  );
};

export { GameOverCard };
