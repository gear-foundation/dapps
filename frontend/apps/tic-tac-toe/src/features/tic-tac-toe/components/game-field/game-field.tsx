import { stateChangeLoadingAtom } from '../../store';
import { useAccount, useAlert } from '@gear-js/react-hooks';
import clsx from 'clsx';
import { motion } from 'framer-motion';
import { useEzTransactions } from 'gear-ez-transactions';
import { useAtom } from 'jotai';
import { BaseComponentProps } from '@/app/types';
import { GameInstance } from '@/app/utils';

import { useGame } from '../../hooks';
import { useEventGameFinishedSubscription, useEventMoveMadeSubscription, useTurnMessage } from '../../sails';
import { calculateWinner } from '../../utils';
import { variantsGameMark } from '../../variants';
import { GameCell } from '../game-cell';
import { GameMark } from '../game-mark';

import styles from './game-field.module.scss';

type GameFieldProps = BaseComponentProps & {
  game: GameInstance;
};

export function GameField({ game }: GameFieldProps) {
  const { turnMessage } = useTurnMessage();
  const { gasless } = useEzTransactions();
  const { countdown } = useGame();
  const board = game.board;
  const [isLoading, setIsLoading] = useAtom(stateChangeLoadingAtom);
  const { account } = useAccount();
  const alert = useAlert();

  useEventMoveMadeSubscription();
  useEventGameFinishedSubscription();

  const winnerRow = calculateWinner(board);
  const winnerColor = winnerRow ? game.player_mark === board[winnerRow[0][0]] : false;

  const onSelectCell = async (step: number) => {
    if (!account) {
      return;
    }

    setIsLoading(true);
    try {
      await turnMessage(step);
    } catch (error) {
      console.log(error);
      alert.error((error instanceof Error && error.message) || 'Game turn error');
      setIsLoading(false);
    }
  };

  return (
    <div
      className={clsx(
        styles.grid,
        // pending && styles.pending
      )}>
      {board.map((mark, i) => (
        <GameCell
          key={i}
          value={i}
          disabled={Boolean(mark || winnerRow?.length) || !countdown?.isActive || !!game.game_result}
          isLoading={isLoading || gasless.isLoading}
          onSelectCell={onSelectCell}>
          {mark && <GameMark mark={mark} className={clsx(styles.mark, mark === game.player_mark && styles.active)} />}
        </GameCell>
      ))}
      {winnerRow && (
        <motion.div
          initial="enter"
          animate="center"
          variants={variantsGameMark}
          className={clsx(styles.line, styles[winnerRow[1]], winnerColor && styles['line--primary'])}
        />
      )}
    </div>
  );
}
