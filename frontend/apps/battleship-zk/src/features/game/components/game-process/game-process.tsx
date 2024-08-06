import { useEffect, useState } from 'react';
import { useAccount } from '@gear-js/react-hooks';
import { Button } from '@gear-js/vara-ui';
import { useEzTransactions } from '@dapps-frontend/ez-transactions';
import { Text } from '@/components/ui/text';
import { GameEndModal, Map } from '@/features/game';
import styles from './GameProcess.module.scss';
import { MapEnemy } from '../map';
import { usePending } from '../../hooks';
import { useCheckBalance } from '@dapps-frontend/hooks';
import { useShips } from '@/features/zk/hooks/use-ships';
import { getFormattedTime } from '../../utils';
import { SHIP_LENGTHS } from '../../consts';
import { GameType, RenderShips } from '../../types';
import { Timer } from '../timer';
import { VerificationModal } from '@/features/game/components/verification-modal';
import YourTurnModal from '../your-turn-modal/your-turn-modal';

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
  onExitGame: () => Promise<void>;
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
  const { signless, gasless } = useEzTransactions();
  const [playerShips, setPlayerShips] = useState<string[]>([]);
  const [enemiesShips, setEnemiesShips] = useState<string[]>([]);
  const [enemiesDeadShips, setEnemiesDeadShips] = useState<number[]>([]);
  const { setPending, pending } = usePending();
  const { checkBalance } = useCheckBalance({
    signlessPairVoucherId: signless.voucher?.id,
    gaslessVoucherId: gasless.voucherId,
  });
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
          console.log(error);
        });
    }
  };

  const onVerifyHit = () => {
    setPending(true);
    onVerifyOponentsHit().catch((error) => {
      console.log(error);
      setPending(false);
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
              {gameType === 'single' || isYourTurn ? 'Your Turn:' : `Enemy's Turn:`}
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
