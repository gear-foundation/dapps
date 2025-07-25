import { useAccount, useAlert } from '@gear-js/react-hooks';
import { Button } from '@gear-js/vara-ui';
import { useEzTransactions } from 'gear-ez-transactions';
import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { TransactionBuilder } from 'sails-js';

import { getErrorMessage } from '@dapps-frontend/ui';

import { ROUTES } from '@/app/consts';
import { Heading } from '@/components/ui/heading';
import { Text } from '@/components/ui/text';
import { useProofShipArrangement } from '@/features/zk/hooks/use-proof-ship-arrangement';
import { useShips } from '@/features/zk/hooks/use-ships';
import { ZkProofData } from '@/features/zk/types';

import { Map } from '../';
import { usePending } from '../../hooks';
import { convertShipsToField } from '../../utils';

import styles from './ShipArrangement.module.scss';
import { generateShipsField } from './shipGenerator';

type GameType = 'single' | 'multi';
interface Props {
  gameType: GameType;
  savedBoard?: string[] | null;
  makeStartGameTransaction: (zkProofData: ZkProofData) => Promise<TransactionBuilder<null>>;
  triggerGame: () => void;
}

export default function ShipArrangement({ gameType, savedBoard, makeStartGameTransaction, triggerGame }: Props) {
  const { account } = useAccount();
  const navigate = useNavigate();
  const { gasless } = useEzTransactions();
  const alert = useAlert();
  const { setPlayerShips, setBoard, createPlayerHits } = useShips();
  const { pending, setPending } = usePending();
  const { requestZKProof } = useProofShipArrangement();
  const [shipsBoard, setShipsBoard] = useState<string[]>([]);
  const [shipsField, setShipsField] = useState<number[][]>([]);
  const [isLoadingGenerate, setLoadingGenerate] = useState(false);

  const onGenerateRandomLayout = async () => {
    setLoadingGenerate(true);
    const newLayout = await generateShipsField(5, 5);
    const playerShipsLayout = convertShipsToField(newLayout, 5, 5);

    if (newLayout !== null) {
      setShipsBoard(playerShipsLayout);
      setShipsField(newLayout.reverse());
      setLoadingGenerate(false);
    }
  };

  useEffect(() => {
    if (savedBoard) {
      setShipsBoard(savedBoard);
    }
  }, [savedBoard]);

  const handleGoBack = () => {
    navigate(ROUTES.HOME);
  };

  const onGameStart = async () => {
    if (!account?.address) {
      return;
    }

    setPending(true);

    try {
      const zkProofData = await requestZKProof(shipsField);

      const transaction = await makeStartGameTransaction(zkProofData);

      const { response } = await transaction.signAndSend();

      await response();

      setPlayerShips(gameType, shipsField);
      createPlayerHits(gameType);
      setBoard(gameType, 'player', shipsBoard);
      setBoard(gameType, 'enemy', convertShipsToField([], 5, 5, 'Unknown'));

      await triggerGame();
    } catch (error) {
      console.error(error);
      alert.error(getErrorMessage(error));
    } finally {
      setPending(false);
    }
  };

  return (
    <div className={styles.content}>
      <div className={styles.header}>
        <Heading>Your Ships</Heading>
        <div className={styles.textWrapper}>
          <Text size="lg">Click 'Generate' to choose a ship arrangement on the board.</Text>
        </div>
      </div>
      <div className={styles.map}>
        <div>
          <Map sizeBlock={72} shipStatusArray={shipsBoard} />
        </div>
      </div>
      <div className={styles.buttons}>
        <Button color="grey" size="small" onClick={handleGoBack} disabled={pending}>
          Back
        </Button>
        <Button
          color="contrast"
          size="small"
          text="Generate"
          onClick={onGenerateRandomLayout}
          disabled={isLoadingGenerate || pending}
        />
        <Button
          text="Continue"
          size="small"
          onClick={onGameStart}
          disabled={!shipsBoard.length || gasless.isLoading}
          isLoading={pending}
        />
      </div>
    </div>
  );
}
