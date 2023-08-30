import { ColumnLeft, ColumnRight, ColumnsContainer } from '../ui/columns'
import { HelpDescription } from '../ui/typography'
import styles from './game.module.scss'
import { GameField } from '../game-field'
import { GameInfoPlayerMark } from '../game-info-player-mark'
import type { IGameInstance, IGameState } from '../../types'
import { GameCountdown } from '../game-countdown'
import { GameTurnButton } from '../game-turn-button'
import { GameStartButton } from '../game-start-button'
import { GameReward } from '../game-reward'
import { Heading } from '@/components/ui/heading'
import { TextGradient } from '@/components/ui/text-gradient'

type GameProps = BaseComponentProps & {
  game: IGameInstance
  config: IGameState['config']
}

export function Game({ game, config }: GameProps) {
  const { gameResult, playerMark } = game
  const { tokensOnLose, tokensOnWin, tokensOnDraw } = config
  return (
    <ColumnsContainer>
      <ColumnLeft>
        <Heading>
          <TextGradient>
            {!!gameResult ? (
              <>
                {gameResult === 'Player' && 'You win'}
                {gameResult === 'Bot' && 'You lose'}
                {gameResult === 'Draw' && "It's a draw"}
              </>
            ) : (
              'Tic Tac Toe game'
            )}
          </TextGradient>
        </Heading>
        <HelpDescription>
          {!!gameResult ? (
            <>
              {gameResult === 'Player' && (
                <p>
                  Congratulations, the game is over, you won! Play and win to
                  make it to the Leaderboard. Good job.
                </p>
              )}
              {gameResult === 'Bot' && (
                <p>
                  Try playing again to win and earn PPV. Play and win to make it
                  to the Leaderboard.
                </p>
              )}
              {gameResult === 'Draw' && (
                <p>
                  The game is over, it's a draw! <br /> Play and win to make it
                  to the Leaderboard. <br /> Try again to win.
                </p>
              )}
            </>
          ) : (
            <p>
              Players take turns making their moves. <br /> Make sure to
              complete your turn before the timer runs out.
            </p>
          )}
        </HelpDescription>

        {!!gameResult && (
          <GameReward
            amount={
              gameResult === 'Player'
                ? tokensOnWin
                : gameResult === 'Draw'
                ? tokensOnDraw
                : tokensOnLose
            }
          />
        )}

        {!gameResult ? (
          <>
            <GameCountdown game={game} />
            <GameTurnButton />
          </>
        ) : (
          <div className={styles.winner}>
            <GameStartButton>Play again</GameStartButton>
          </div>
        )}
      </ColumnLeft>
      <ColumnRight className={styles.field}>
        <GameField game={game} />

        <GameInfoPlayerMark
          isNewGame={!gameResult}
          mark={playerMark}
          className={styles.choose}
        />
      </ColumnRight>
    </ColumnsContainer>
  )
}
