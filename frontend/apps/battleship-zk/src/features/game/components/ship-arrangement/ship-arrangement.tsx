import { useEffect, useState } from 'react';
import { useEzTransactions } from '@dapps-frontend/ez-transactions';
import { Button } from '@gear-js/vara-ui';
import { useAccount } from '@gear-js/react-hooks';
import { Heading } from '@/components/ui/heading';
import { Text } from '@/components/ui/text';
import { Map } from '../';
import styles from './ShipArrangement.module.scss';
import { usePending } from '../../hooks';
import { generateShipsField } from './shipGenerator';
import { convertShipsToField } from '../../utils';
import { useCheckBalance } from '@dapps-frontend/hooks';
import { useShips } from '@/features/zk/hooks/use-ships';
import { useProofShipArrangement } from '@/features/zk/hooks/use-proof-ship-arrangement';
import { ZkProofData } from '@/features/zk/types';
import { TransactionBuilder } from 'sails-js';
import { useNavigate } from 'react-router-dom';
import { useSingleplayerGame } from '@/features/singleplayer/hooks/use-singleplayer-game';

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
  const { gasless, signless } = useEzTransactions();
  const { setPlayerShips, setBoard, createPlayerHits } = useShips();
  const { pending, setPending } = usePending();
  const { checkBalance } = useCheckBalance({
    signlessPairVoucherId: signless.voucher?.id,
    gaslessVoucherId: gasless.voucherId,
  });
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
    navigate('/');
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
      console.log(error);
    } finally {
      setPending(false);
    }
  };

  return (
    <div className={styles.content}>
      <div className={styles.header}>
        <Heading>Your ships</Heading>
        <div className={styles.textWrapper}>
          <Text size="lg">Choose a ship placement scheme, and to see a new arrangement, click "Generate"</Text>
        </div>
      </div>
      <div style={{ width: '100%' }}>
        <div>
          <Map sizeBlock={72} shipStatusArray={shipsBoard} />
        </div>
      </div>
      <div className={styles.buttons}>
        <Button color="grey" onClick={handleGoBack} disabled={pending}>
          Back
        </Button>
        <Button color="dark" text="Generate" onClick={onGenerateRandomLayout} disabled={isLoadingGenerate || pending} />
        <Button
          text="Continue"
          onClick={onGameStart}
          disabled={!shipsBoard.length || gasless.isLoading}
          isLoading={pending}
        />
      </div>
    </div>
  );
}
