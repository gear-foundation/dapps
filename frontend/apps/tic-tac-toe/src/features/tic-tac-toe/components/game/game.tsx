import { EzTransactionsSwitch } from 'gear-ez-transactions';

import { SIGNLESS_ALLOWED_ACTIONS } from '@/app/consts';
import { BaseComponentProps } from '@/app/types';
import { GameInstance } from '@/app/utils';
import { Heading } from '@/components/ui/heading';
import { TextGradient } from '@/components/ui/text-gradient';
import { useGame } from '@/features/tic-tac-toe/hooks';

import { GameCountdown } from '../game-countdown';
import { GameField } from '../game-field';
import { GameInfoPlayerMark } from '../game-info-player-mark';
import { GameSkipButton } from '../game-skip-button';
import { GameStartButton } from '../game-start-button';
import { HelpDescription } from '../ui/typography';

import styles from './game.module.scss';

type GameProps = BaseComponentProps & {
  game: GameInstance;
};

export function Game({ game }: GameProps) {
  const { game_result, player_mark } = game;
  const { countdown } = useGame();

  return (
    <section className={styles.game}>
      <Heading className={styles.game__heading}>
        <>
          {game_result ? (
            <>
              {game_result === 'Player' && <TextGradient>You win</TextGradient>}
              {game_result === 'Bot' && <TextGradient className={styles.loose}>You lose</TextGradient>}
              {game_result === 'Draw' && "It's a draw"}
            </>
          ) : (
            <TextGradient>Tic Tac Toe game</TextGradient>
          )}
        </>
      </Heading>
      <HelpDescription className={styles.game__text}>
        {game_result ? (
          <>
            {game_result === 'Player' && (
              <p>
                Congratulations, the game is over, you won! <br /> Good job.
              </p>
            )}
            {game_result === 'Bot' && <p>Try again to win.</p>}
            {game_result === 'Draw' && (
              <p>
                The game is over, it&apos;s a draw! <br />
                Try again to win.
              </p>
            )}
          </>
        ) : (
          <p>
            Players take turns making their moves. <br /> Make sure to complete your turn before the timer runs out.
          </p>
        )}
      </HelpDescription>

      <div className={styles.game__actions}>
        {!game_result ? (
          <>
            {countdown?.isActive ? (
              <GameCountdown game={game} className={styles.game__countdown} />
            ) : (
              <GameSkipButton />
            )}
          </>
        ) : (
          <div className={styles.game__play}>
            <GameStartButton>Play again</GameStartButton>
            <EzTransactionsSwitch allowedActions={SIGNLESS_ALLOWED_ACTIONS} />
          </div>
        )}
      </div>

      <div className={styles.game__field}>
        <GameField game={game} />

        <GameInfoPlayerMark isNewGame={!game_result} mark={player_mark} className={styles.choose} />
      </div>
    </section>
  );
}
