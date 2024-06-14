import { useState } from 'react';
import { useEzTransactions } from '@dapps-frontend/ez-transactions';
import { Button } from '@gear-js/vara-ui';
import { useAccount } from '@gear-js/react-hooks';
import { Heading } from '@/components/ui/heading';
import { Text } from '@/components/ui/text';
import { Map } from '../';
import styles from './ShipArrangement.module.scss';
import { useGame, useGameMessage, usePending } from '../../hooks';
import { generateShipsField } from './shipGenerator';
import { convertShipsToField } from '../../utils';
import { useCheckBalance } from '@dapps-frontend/hooks';
import { useShips } from '@/features/zk/hooks/use-ships';
// import { buildPoseidonOpt } from 'circomlibjs';
// @ts-ignore
import { buildPoseidon } from '@/features/zk/utils/poseidon';
import { sails } from '@/app/utils/sails';
import { ADDRESS } from '@/app/consts';
import { web3FromSource } from '@polkadot/extension-dapp';
import { useProofShipArrangement } from '@/features/zk/hooks/use-proof-ship-arrangement';

interface ReducedShips {
  [key: string]: number[];
}

export default function ShipArrangement() {
  const { account } = useAccount();
  const { gasless, signless } = useEzTransactions();
  const { setPlayerShips, setBoard } = useShips();
  const { triggerGame } = useGame();
  const message = useGameMessage();
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
      setShipsField(newLayout);
      setLoadingGenerate(false);
    }
  };

  const onGameStart = async () => {
    if (!account?.address) {
      return;
    }

    setPending(true);

    try {
      const { proofContent, publicContent } = await requestZKProof(shipsField);

      const injector = await web3FromSource(account.meta.source);

      const startSingleGame = sails.services.Single.functions.StartSingleGame(proofContent, {
        hash: publicContent.publicHash,
      });

      const transaction = await startSingleGame
        .withAccount(account.address, { signer: injector.signer })
        .withGas(250_000_000_000n);

      await transaction.signAndSend();

      setPlayerShips(shipsField);
      setBoard('player', shipsBoard);
      setBoard('enemy', convertShipsToField([], 5, 5, 'Unknown'));

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
        <Button color="dark" text="Generate" onClick={onGenerateRandomLayout} disabled={isLoadingGenerate} />
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
