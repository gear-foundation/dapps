import { Card, Text } from '@/components';
import styles from './game-over-card.module.scss';
import { VaraIcon } from '@/components/layout';
import clsx from 'clsx';

type GameResult = 'win' | 'draw' | 'lose';

type GameOverCardProps = {
  prizeCount: number;
  className?: string;
  result: GameResult;
  isTournamentOver?: boolean;
  player1name: string;
  player2name: string;
};

const GameOverCard = ({
  prizeCount,
  result,
  className,
  player1name,
  player2name,
  isTournamentOver,
}: GameOverCardProps) => {
  const resultTexts = {
    win: 'You win',
    draw: 'It’s a draw',
    lose: 'You lose',
  };
  const descriptionTexts = {
    win: `${player1name} wins!`,
    draw: `${player1name} and ${player2name} ended in a draw!`,
    lose: `${player2name} wins! Now you can watch other players' battles. Choose any battle from the list below.`,
  };

  return (
    <div className={clsx(styles.backdrop, className)}>
      <Card title="Game over" description={descriptionTexts[result]} className={styles.card} size="sm">
        {isTournamentOver && (
          <div className={styles.prize}>
            <Text size="sm">Winner prize:</Text>
            <VaraIcon className={styles.icon} />
            <Text size="sm" weight="semibold">
              {prizeCount} VARA
            </Text>
          </div>
        )}
      </Card>
      <Text weight="bold" className={styles.result}>
        {resultTexts[result]}
      </Text>
    </div>
  );
};

export { GameOverCard };
