import clsx from 'clsx';
import styles from './game-field.module.scss';
import { GameCell } from '../game-cell';
import type { IGameInstance } from '../../types';
import { GameMark } from '../game-mark';
import { useCheckGaslessVouher, useGame, useGameMessage, useSubscriptionOnGameMessage } from '../../hooks';
import { calculateWinner } from '../../utils';
import { motion } from 'framer-motion';
import { variantsGameMark } from '../../variants';
import { BaseComponentProps } from '@/app/types';
import { useEffect } from 'react';
import { useAtom } from 'jotai';
import { stateChangeLoadingAtom } from '../../store';
import { useAccount, useAlert, useHandleCalculateGas } from '@gear-js/react-hooks';
import { ADDRESS } from '../../consts';
import { useCheckBalance } from '@dapps-frontend/hooks';
import { useEzTransactions } from '@dapps-frontend/ez-transactions';
import { withoutCommas } from '@/app/utils';
import { ProgramMetadata } from '@gear-js/api';

type GameFieldProps = BaseComponentProps & {
  game: IGameInstance;
  meta: ProgramMetadata;
};

export function GameField({ game, meta }: GameFieldProps) {
  const { signless, gasless } = useEzTransactions();
  const { countdown } = useGame();
  const [isLoading, setIsLoading] = useAtom(stateChangeLoadingAtom);
  const board = game.board;
  const { account } = useAccount();
  const alert = useAlert();
  const calculateGas = useHandleCalculateGas(ADDRESS.GAME, meta);
  const message = useGameMessage(meta);
  const { checkBalance } = useCheckBalance({
    signlessPairVoucherId: signless.voucher?.id,
    gaslessVoucherId: gasless.voucherId,
  });
  const { subscribe, unsubscribe, isOpened } = useSubscriptionOnGameMessage(meta);

  const winnerRow = calculateWinner(board);
  const winnerColor = winnerRow ? game.playerMark === board[winnerRow[0][0]] : false;

  const onSelectCell = async (value: number) => {
    if (!meta || !account || !ADDRESS.GAME) {
      return;
    }

    const payload = { Turn: { step: value } };

    if (!isLoading) {
      calculateGas(payload)
        .then((res) => res.toHuman())
        .then(({ min_limit }) => {
          const minLimit = withoutCommas(min_limit as string);
          const gasLimit = Math.floor(Number(minLimit) + Number(minLimit) * 0.2);

          subscribe();

          checkBalance(
            gasLimit,
            () =>
              message({
                payload,
                gasLimit,
                voucherId: gasless.voucherId,
                onError: () => {
                  unsubscribe();
                },
                onSuccess: () => {
                  console.log('success on cell');
                },
              }),
            () => {
              unsubscribe();
            },
          );
        })
        .catch((error) => {
          console.log(error);
          alert.error('Gas calculation error');
        });
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  };

  useEffect(() => {
    setIsLoading(isOpened);
  }, [isOpened]);

  const checkGaslessVoucher = useCheckGaslessVouher<number>(onSelectCell);

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
          disabled={Boolean(mark || winnerRow?.length) || !countdown?.isActive || !!game.gameResult}
          isLoading={isLoading || gasless.isLoading}
          onSelectCell={checkGaslessVoucher}>
          {mark && <GameMark mark={mark} className={clsx(styles.mark, mark === game.playerMark && styles.active)} />}
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
