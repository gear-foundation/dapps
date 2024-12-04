import { HelpDescription } from '../ui/typography';
import styles from './game.module.scss';
import { GameField } from '../game-field';
import { GameInfoPlayerMark } from '../game-info-player-mark';
import type { IGameInstance } from '../../types';
import { GameCountdown } from '../game-countdown';
import { GameSkipButton } from '../game-skip-button';
import { GameStartButton } from '../game-start-button';
import { Heading } from '@/components/ui/heading';
import { TextGradient } from '@/components/ui/text-gradient';
import { useGame } from '@/features/tic-tac-toe/hooks';
import { BaseComponentProps } from '@/app/types';
import { ProgramMetadata } from '@gear-js/api';
import { EzTransactionsSwitch } from 'gear-ez-transactions';
import { SIGNLESS_ALLOWED_ACTIONS } from '@/app/consts';

type GameProps = BaseComponentProps & {
  game: IGameInstance;
  meta: ProgramMetadata;
};

export function Game({ game, meta }: GameProps) {
  const { gameResult, playerMark } = game;
  const { countdown } = useGame();

  return (
    <section className={styles.game}>
      <Heading className={styles.game__heading}>
        <>
          {!!gameResult ? (
            <>
              {gameResult === 'Player' && <TextGradient>You win</TextGradient>}
              {gameResult === 'Bot' && <TextGradient className={styles.loose}>You lose</TextGradient>}
              {gameResult === 'Draw' && "It's a draw"}
            </>
          ) : (
            <TextGradient>Tic Tac Toe game</TextGradient>
          )}
        </>
      </Heading>
      <HelpDescription className={styles.game__text}>
        {!!gameResult ? (
          <>
            {gameResult === 'Player' && (
              <p>
                Congratulations, the game is over, you won! <br /> Good job.
              </p>
            )}
            {gameResult === 'Bot' && <p>Try again to win.</p>}
            {gameResult === 'Draw' && (
              <p>
                The game is over, it's a draw! <br />
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
        {Boolean(!gameResult) ? (
          <>
            {countdown?.isActive ? (
              <GameCountdown game={game} className={styles.game__countdown} />
            ) : (
              <GameSkipButton meta={meta} />
            )}
          </>
        ) : (
          <div className={styles.game__play}>
            <GameStartButton meta={meta}>Play again</GameStartButton>
            <EzTransactionsSwitch allowedActions={SIGNLESS_ALLOWED_ACTIONS} />
          </div>
        )}
      </div>

      <div className={styles.game__field}>
        <GameField game={game} meta={meta} />

        <GameInfoPlayerMark isNewGame={!gameResult} mark={playerMark} className={styles.choose} />
      </div>
    </section>
  );
}
