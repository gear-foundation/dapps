import { Button } from '@gear-js/vara-ui';
import { useEzTransactions } from 'gear-ez-transactions';
import { useState } from 'react';

import { useCheckBalance } from '@dapps-frontend/hooks';

import { Heading } from '@/components/ui/heading';
import { Text } from '@/components/ui/text';

import { Map } from '../';
import { useGameMessage, usePending } from '../../hooks';
import { convertShipsToField } from '../../utils';

import styles from './ShipArrangement.module.scss';
import { generateShipsField } from './shipGenerator';

export default function ShipArrangement() {
  const { gasless, signless } = useEzTransactions();

  const message = useGameMessage();
  const { setPending } = usePending();
  const { checkBalance } = useCheckBalance({
    signlessPairVoucherId: signless.voucher?.id,
    gaslessVoucherId: gasless.voucherId,
  });

  const [shipLayout, setShipLayout] = useState<string[]>([]);
  const [shipsField, setShipsField] = useState<number[][]>([]);
  const [isLoadingGenerate, setLoadingGenerate] = useState(false);

  const onGenerateRandomLayout = () => {
    setLoadingGenerate(true);
    const newLayout = generateShipsField(5, 5);
    const playerShipsLayout = convertShipsToField(newLayout, 5, 5);

    if (newLayout !== null) {
      setShipLayout(playerShipsLayout);
      setShipsField(newLayout);
      setLoadingGenerate(false);
    }
  };

  const onGameStart = () => {
    const gasLimit = 120000000000;

    if (!gasless.isLoading) {
      setPending(true);

      checkBalance(gasLimit, () =>
        message({
          payload: {
            StartGame: {
              ships: shipsField,
            },
          },
          voucherId: gasless.voucherId,
          gasLimit,
        }),
      );
    }
  };

  return (
    <div className={styles.content}>
      <div className={styles.header}>
        <Heading>Your ships</Heading>
        <div className={styles.textWrapper}>
          <Text size="lg">
            Choose a ship placement scheme, and to see a new arrangement, click &quot;Generate&quot;
          </Text>
        </div>
      </div>
      <div style={{ width: '100%' }}>
        <div>
          <Map sizeBlock={72} shipStatusArray={shipLayout} />
        </div>
      </div>
      <div className={styles.buttons}>
        <Button color="contrast" text="Generate" onClick={onGenerateRandomLayout} disabled={isLoadingGenerate} />
        <Button text="Continue" onClick={onGameStart} disabled={!shipLayout.length || gasless.isLoading} />
      </div>
    </div>
  );
}
