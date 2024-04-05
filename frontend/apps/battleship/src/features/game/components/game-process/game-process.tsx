import { useEffect, useState } from 'react';
import { useEzTransactions } from '@dapps-frontend/ez-transactions';
import { Text } from '@/components/ui/text';
import { GameEndModal, Map } from '@/features/game';
import styles from './GameProcess.module.scss';
import { MapEnemy } from '../map';
import { useGame, useGameMessage, usePending } from '../../hooks';
import { getFormattedTime } from '../../utils';
import { Loader } from '@/components';
import { useCheckBalance } from '@dapps-frontend/hooks';

export default function GameProcess() {
  const { signless, gasless } = useEzTransactions();
  const [playerShips, setPlayerShips] = useState<string[]>([]);
  const [enemiesShips, setEnemiesShips] = useState<string[]>([]);
  const [elapsedTime, setElapsedTime] = useState('');
  const [totalGameTime, setTotalGameTime] = useState('');
  const [isDisabledCell, setDisabledCell] = useState(false);

  const { gameState } = useGame();
  const { setPending } = usePending();
  const message = useGameMessage();
  const { checkBalance } = useCheckBalance({
    signlessPairVoucherId: signless.voucher?.id,
    gaslessVoucherId: gasless.voucherId,
  });

  const [isOpenEndModal, setIsOpenEndModal] = useState(false);
  const openEndModal = () => setIsOpenEndModal(true);
  const closeEndModal = () => setIsOpenEndModal(false);

  const totalShips = gameState?.botShips.reduce((total, [, shipCount]) => {
    return total + parseInt(shipCount, 10);
  }, 0);
  const totalShoots = gameState ? parseInt(gameState.totalShots) : 0;
  const successfulShoots = gameState?.botBoard.filter((x) => x === 'DeadShip' || x === 'BoomShip').length ?? 0;
  const efficiency = totalShoots !== 0 ? ((successfulShoots / totalShoots) * 100).toFixed(2) : 0;

  useEffect(() => {
    if (gameState) {
      let gameOverHandled = false;

      const updateTimer = () => {
        const currentTime = new Date().getTime();
        const startTime = parseInt(gameState.startTime.replace(/,/g, ''));
        const elapsedTimeMilliseconds = currentTime - startTime;

        const formattedTime = getFormattedTime(elapsedTimeMilliseconds);

        !gameState.gameOver && setElapsedTime(formattedTime);

        if (gameState.gameOver && !gameOverHandled) {
          const endTime = parseInt(gameState.endTime.replace(/,/g, ''));
          const elapsedTimeMilliseconds = endTime - startTime;
          const formattedTime = getFormattedTime(elapsedTimeMilliseconds);

          setElapsedTime(formattedTime);
          setTotalGameTime(formattedTime);
          openEndModal();

          gameOverHandled = true;
        }
      };

      const timerInterval = setInterval(updateTimer, 1000);

      return () => {
        clearInterval(timerInterval);
      };
    }
  }, [gameState]);

  const onClickCell = async (indexCell: number) => {
    const gasLimit = 120000000000;

    if (!gasless.isLoading) {
      setDisabledCell(true);

      checkBalance(
        gasLimit,
        () =>
          message({
            payload: { Turn: { step: indexCell } },
            onInBlock: (messageId) => {
              if (messageId) {
                setDisabledCell(false);
              }
            },
            gasLimit,
            voucherId: gasless.voucherId,
            onSuccess: () => {
              setPending(false);
            },
            onError: () => {
              setDisabledCell(false);
            },
          }),
        () => setDisabledCell(false),
      );
    }
  };

  useEffect(() => {
    if (gameState?.playerBoard) {
      setPlayerShips(gameState.playerBoard);
    }

    if (gameState?.botBoard) {
      setEnemiesShips(gameState.botBoard);
    }
  }, [gameState]);

  if (!gameState) {
    return <Loader />;
  }

  return (
    <div className={styles.container}>
      <div className={styles.content}>
        <div>
          <Map sizeBlock={32} shipStatusArray={playerShips} />
        </div>
        <div className={styles.gameInfo}>
          <Text size="sm" weight="normal">
            Time: <span>{elapsedTime}</span>
          </Text>
          <Text size="sm" weight="normal">
            Total shots: <span>{totalShoots}</span>
          </Text>
          <Text size="sm" weight="normal">
            Successful hits: <span>{successfulShoots}</span>
          </Text>
          <Text size="sm" weight="normal">
            Efficiency: <span>{efficiency}%</span>
          </Text>
        </div>
      </div>
      <div className={styles.enemyShips}>
        <Text size="sm" weight="normal" className={styles.text}>
          Enemy Ships: {totalShips} / 4
        </Text>

        <div className={styles.listShips}>
          {gameState?.botShips.map(([shipType, shipCount], index) => {
            const numberOfBlocks = parseInt(shipType, 10);
            const shipsToRender = Array.from({
              length: parseInt(shipCount, 10),
            });

            return (
              <div key={index} className={styles.ship}>
                {shipsToRender.map((_, shipIndex) => (
                  <div key={shipIndex} className={styles.ship}>
                    {[...Array(numberOfBlocks)].map((_, blockIndex) => (
                      <div key={blockIndex} className={styles.block}></div>
                    ))}
                  </div>
                ))}
              </div>
            );
          })}
        </div>
      </div>

      <div>
        <MapEnemy
          sizeBlock={86}
          onClickCell={onClickCell}
          shipStatusArray={enemiesShips}
          isDisabledCell={isDisabledCell || gasless.isLoading || gameState.gameOver}
        />
      </div>

      {isOpenEndModal && gameState && (
        <GameEndModal
          onClose={closeEndModal}
          time={totalGameTime}
          totalShoots={totalShoots}
          successfulShoots={successfulShoots}
          efficiency={efficiency}
          gameResult={gameState.gameResult}
        />
      )}
    </div>
  );
}
