import { useAccount, useAlert } from '@gear-js/react-hooks';
import { Button } from '@gear-js/vara-ui';
import { useEzTransactions } from 'gear-ez-transactions';
import { useEffect, useState } from 'react';

import { getErrorMessage } from '@dapps-frontend/ui';

import { Text } from '@/components/ui/text';
import { GameEndModal, Map } from '@/features/game';
import { VerificationModal } from '@/features/game/components/verification-modal';
import { useShips } from '@/features/zk/hooks/use-ships';

import { SHIP_LENGTHS } from '../../consts';
import { usePending } from '../../hooks';
import { GameType, RenderShips } from '../../types';
import { getFormattedTime } from '../../utils';
import { MapEnemy } from '../map';
import { Timer } from '../timer';
import YourTurnModal from '../your-turn-modal/your-turn-modal';

import styles from './GameProcess.module.scss';

type GameUpdatedEvent = {
  turn: string;
  pendingVerification?: string;
  verificationRequired?: number | null;
};

type GameResults = {
  totalTime: string | number | bigint | undefined;
  winner: string;
};

type Props = {
  gameType: GameType;
  totalShoots: number;
  successfulShoots: number;
  gameResults: GameResults | null;
  gameUpdatedEvent: GameUpdatedEvent;
  remainingTime: string | number | bigint | null | undefined;
  admin: string | undefined;
  onClickCell: (handleClickCell: number) => Promise<void>;
  onVerifyOponentsHit: () => Promise<void>;
  onExitGame: () => void;
  resetGameState: () => void;
};

export default function GameProcess({
  gameType,
  totalShoots,
  successfulShoots,
  gameUpdatedEvent,
  remainingTime,
  gameResults,
  admin,
  onClickCell,
  onVerifyOponentsHit,
  onExitGame,
  resetGameState,
}: Props) {
  const { account } = useAccount();
  const alert = useAlert();
  const { gasless } = useEzTransactions();
  const [playerShips, setPlayerShips] = useState<string[]>([]);
  const [enemiesShips, setEnemiesShips] = useState<string[]>([]);
  const [enemiesDeadShips, setEnemiesDeadShips] = useState<number[]>([]);
  const { setPending, pending } = usePending();
  const { getBoard, checkIsStepOnShip } = useShips();
  const [isOpenEndModal, setIsOpenEndModal] = useState(false);
  const openEndModal = () => setIsOpenEndModal(true);
  const closeEndModal = () => setIsOpenEndModal(false);
  const [playerLastHit, setPlayerLastHit] = useState<number | null>(null);

  const { verificationRequired, pendingVerification, turn } = gameUpdatedEvent;
  const { isBoomShip, isDeadShip } = checkIsStepOnShip(gameType, verificationRequired) || {};
  const isVerificationRequired = isBoomShip || isDeadShip;

  const isYourTurn =
    (gameType === 'single' && !isVerificationRequired && !pending) ||
    turn === account?.decodedAddress ||
    pendingVerification === account?.decodedAddress;

  const showMapTimer = gameType === 'single' ? false : !isYourTurn;

  const efficiency = totalShoots !== 0 ? ((successfulShoots / totalShoots) * 100).toFixed(2) : 0;

  const handleClickCell = (indexCell: number) => {
    if (!gasless.isLoading) {
      setPending(true);
      onClickCell(indexCell)
        .then(() => setPlayerLastHit(indexCell))
        .catch((error) => {
          setPending(false);
          console.error(error);
          alert.error(getErrorMessage(error));
        });
    }
  };

  const onVerifyHit = () => {
    setPending(true);
    onVerifyOponentsHit().catch((error) => {
      setPending(false);
      console.error(error);
      alert.error(getErrorMessage(error));
    });
  };

  const handleDefineDeadShips = (deadShips: RenderShips) => {
    setEnemiesDeadShips(Object.values(deadShips).map((item) => item.length));
  };

  useEffect(() => {
    const boardPlayer = getBoard(gameType, 'player');

    if (boardPlayer) {
      setPlayerShips(boardPlayer);
    }

    const boardEnemy = getBoard(gameType, 'enemy');

    if (boardEnemy) {
      setEnemiesShips(boardEnemy);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [gameUpdatedEvent]);

  useEffect(() => {
    if (gameResults) {
      openEndModal();
    }
  }, [gameResults]);

  const generateShipBlocks = () => {
    const deadShips = [...enemiesDeadShips];

    return SHIP_LENGTHS.map((numberOfBlocks, index) => {
      if (deadShips.includes(numberOfBlocks)) {
        const blocksToRemoveIndex = deadShips.findIndex((item) => item === numberOfBlocks);

        deadShips.splice(blocksToRemoveIndex, 1);
        return null;
      }

      const blocksToRender = Array.from({
        length: numberOfBlocks,
      });

      return (
        <div key={index} className={styles.ship}>
          {blocksToRender.map((_, blockIndex) => (
            <div key={blockIndex} className={styles.block}></div>
          ))}
        </div>
      );
    });
  };

  return (
    <div className={styles.container}>
      <div className={styles.content}>
        <div>
          <Map sizeBlock={32} shipStatusArray={playerShips} lastHit={verificationRequired} />
        </div>
        <div className={styles.gameInfoWrapper}>
          <div className={styles.gameInfoTurn}>
            <Text size="sm" weight="medium">
              {gameType === 'multi' ? 'Peer-to-peer game' : 'Singleplayer'}
            </Text>
          </div>
          <div className={styles.gameInfo}>
            <Text size="sm" weight="normal">
              {gameType === 'single' || isYourTurn ? 'Your Turn:' : `Enemy&apos;s Turn:`}
              <Timer remainingTime={remainingTime} shouldGoOn={!gameResults} redOnLast />
            </Text>
            <Text size="sm" weight="normal">
              Total Shots: <span>{totalShoots}</span>
            </Text>
            <Text size="sm" weight="normal">
              Successful Hits: <span>{successfulShoots}</span>
            </Text>
            <Text size="sm" weight="normal">
              Efficiency: <span>{efficiency}%</span>
            </Text>
          </div>
        </div>
      </div>
      <div className={styles.enemyShips}>
        <Text size="sm" weight="normal" className={styles.text}>
          Enemy Ships: {SHIP_LENGTHS.length - enemiesDeadShips.length} / {SHIP_LENGTHS.length}
        </Text>

        <div className={styles.listShips}>{generateShipBlocks()}</div>
      </div>

      <div>
        <MapEnemy
          sizeBlock={86}
          onClickCell={handleClickCell}
          shipStatusArray={enemiesShips}
          isDisabledCell={pending || gasless.isLoading || isVerificationRequired || !isYourTurn || !!gameResults}
          onDefineDeadShip={handleDefineDeadShips}
          lastHit={playerLastHit}
          showTimer={showMapTimer}
          remainingTime={remainingTime}
        />
      </div>
      <div className={styles.exitButtonWrapper}>
        {admin === account?.decodedAddress && !gameResults ? (
          <Button className={styles.cancelGameButton} color="grey" onClick={onExitGame}>
            Cancel game
          </Button>
        ) : (
          <Button className={styles.exitButton} color="grey" onClick={onExitGame}>
            Exit
          </Button>
        )}
      </div>
      {isVerificationRequired && !gameResults && (
        <VerificationModal onVerifyHit={onVerifyHit} isDeadShip={isDeadShip} isLoading={pending} onExit={onExitGame} />
      )}
      {isOpenEndModal && gameResults && (
        <GameEndModal
          onClose={closeEndModal}
          resetGameState={resetGameState}
          time={getFormattedTime(Number(gameResults.totalTime))}
          totalShoots={totalShoots}
          successfulShoots={successfulShoots}
          efficiency={efficiency}
          winner={gameResults.winner}
          gameType={gameType}
        />
      )}
      <YourTurnModal isYourTurn={isYourTurn} />
    </div>
  );
}
