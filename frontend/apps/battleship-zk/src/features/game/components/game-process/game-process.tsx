import { useEffect, useMemo, useState } from 'react';
import { useEzTransactions } from '@dapps-frontend/ez-transactions';
import { Text } from '@/components/ui/text';
import { GameEndModal, Map } from '@/features/game';
import styles from './GameProcess.module.scss';
import { MapEnemy } from '../map';
import { useGame, useGameMessage, usePending } from '../../hooks';
import { Loader } from '@/components';
import { useCheckBalance } from '@dapps-frontend/hooks';
import { Account, useAccount, useApi } from '@gear-js/react-hooks';
import { web3FromSource } from '@polkadot/extension-dapp';
import { sails } from '@/app/utils/sails';
import { useProofShipHit } from '@/features/zk/hooks/use-proof-ship-hit';
import { useShips } from '@/features/zk/hooks/use-ships';
import { useEventGameEndSubscription } from '@/app/utils/sails/events/use-event-game-end-subscription';
import { getFormattedTime } from '../../utils';
import { SHIP_LENGTHS } from '../../consts';
import { RenderShips } from '../../types';

type Props = {
  isMultiplayer?: boolean;
};

export default function GameProcess({ isMultiplayer = false }: Props) {
  const { signless, gasless } = useEzTransactions();
  const [playerShips, setPlayerShips] = useState<string[]>([]);
  const [enemiesShips, setEnemiesShips] = useState<string[]>([]);
  const [enemiesDeadShips, setEnemiesDeadShips] = useState<number[]>([]);
  const [elapsedTime, setElapsedTime] = useState('');
  const [isDisabledCell, setDisabledCell] = useState(false);
  const { game, triggerGame } = useGame();
  const { setPending } = usePending();
  const message = useGameMessage();
  const { checkBalance } = useCheckBalance({
    signlessPairVoucherId: signless.voucher?.id,
    gaslessVoucherId: gasless.voucherId,
  });
  const { account } = useAccount();
  const { api } = useApi();
  const { getProofData, clearProofData } = useProofShipHit();
  const { getBoard } = useShips();

  const { result } = useEventGameEndSubscription();

  const [isOpenEndModal, setIsOpenEndModal] = useState(false);
  const openEndModal = () => setIsOpenEndModal(true);
  const closeEndModal = () => setIsOpenEndModal(false);

  const totalShips = Object.entries(game?.bot_ships || {}).reduce((total, [, shipCount]) => {
    return shipCount !== '0x' ? total + 1 : total;
  }, 0);
  const totalShoots = game ? parseInt(game.total_shots) : result?.total_shots || 0;
  const successfulShoots = game ? game.succesfull_shots : result?.succesfull_shots || 0;
  const efficiency = totalShoots !== 0 ? ((successfulShoots / totalShoots) * 100).toFixed(2) : 0;

  useEffect(() => {
    if (game) {
      const updateTimer = () => {
        const currentTime = new Date().getTime();
        const startTime = game.start_time;
        const elapsedTimeMilliseconds = currentTime - startTime;

        const formattedTime = getFormattedTime(elapsedTimeMilliseconds);

        game && !result && setElapsedTime(formattedTime);
      };

      const timerInterval = setInterval(updateTimer, 1000);

      return () => {
        clearInterval(timerInterval);
      };
    }
  }, [game]);

  const onClickCellFinally = () => {
    setDisabledCell(false);
  };

  const getVerifyTransaction = async (account: Account, proofDataHit: any, gasLimit: bigint) => {
    if (!proofDataHit) {
      return null;
    }

    const { proofContent, publicContent } = proofDataHit;

    const injector = await web3FromSource(account.meta.source);

    const verifyMove = sails.services.Single.functions.VerifyMove(proofContent, {
      hash: publicContent.publicHash,
      out: publicContent.results[0][0],
      hit: publicContent.results[1][0],
    });

    const transaction = await verifyMove.withAccount(account.address, { signer: injector.signer }).withGas(gasLimit);

    return transaction;
  };

  const getHitTransaction = async (account: Account, indexCell: number, gasLimit: bigint) => {
    const injector = await web3FromSource(account.meta.source);

    const makeMove = sails.services.Single.functions.MakeMove(indexCell);

    const transaction = await makeMove.withAccount(account.address, { signer: injector.signer }).withGas(gasLimit);

    return transaction;
  };

  const onClickCell = async (indexCell: number) => {
    if (!account?.address || !api) {
      return;
    }

    const gasLimit = 120_000_000_000n;

    if (!gasless.isLoading) {
      setDisabledCell(true);
      const proofDataHit = getProofData();

      try {
        const hitTransaction = await getHitTransaction(account, indexCell, gasLimit);
        const verifyTransaction = await getVerifyTransaction(account, proofDataHit, gasLimit);

        if (verifyTransaction) {
          const { response } = await verifyTransaction.signAndSend();

          clearProofData();

          await response();
        }

        const { response } = await hitTransaction.signAndSend();

        await response();
        await triggerGame();

        onClickCellFinally();
        setPending(false);
      } catch (error) {
        console.log(error);
        onClickCellFinally();
      }

      // checkBalance(
      //   gasLimit,
      //   () =>
      //     message({
      //       payload: { Turn: { step: indexCell } },
      //       onInBlock: (messageId) => {
      //         if (messageId) {
      //           onClickCellFinally();
      //         }
      //       },
      //       gasLimit,
      //       voucherId: gasless.voucherId,
      //       onSuccess: () => {
      //         setPending(false);
      //       },
      //       onError: onClickCellFinally,
      //     }),
      //   onClickCellFinally,
      // );
    }
  };

  const handleDefineDeadShips = (deadShips: RenderShips) => {
    setEnemiesDeadShips(Object.values(deadShips).map((item) => item.length));
  };

  useEffect(() => {
    const boardPlayer = getBoard('player');

    if (boardPlayer) {
      setPlayerShips(boardPlayer);
    }

    const boardEnemy = getBoard('enemy');

    if (boardEnemy) {
      setEnemiesShips(boardEnemy);
    }
  }, [game]);

  useEffect(() => {
    if (result) {
      openEndModal();
    }
  }, [result]);

  const generateShipBlocks = () => {
    const deadShips = [...enemiesDeadShips];

    return SHIP_LENGTHS.reverse().map((numberOfBlocks, index) => {
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

  if (game === undefined) {
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
            Time: <span>{result?.time || elapsedTime}</span>
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

        <div className={styles.listShips}>{generateShipBlocks()}</div>
      </div>

      <div>
        <MapEnemy
          sizeBlock={86}
          onClickCell={onClickCell}
          shipStatusArray={enemiesShips}
          isDisabledCell={isDisabledCell || gasless.isLoading || !game}
          onDefineDeadShip={handleDefineDeadShips}
        />
      </div>

      {isOpenEndModal && result && (
        <GameEndModal
          onClose={closeEndModal}
          time={getFormattedTime(result.time)}
          totalShoots={result.total_shots}
          successfulShoots={result.succesfull_shots}
          efficiency={efficiency}
          gameResult={result.winner}
        />
      )}
    </div>
  );
}
