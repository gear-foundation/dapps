import { useState } from 'react';
import { Button } from '@gear-js/vara-ui';
import { Heading } from '@/components/ui/heading';
import { TextGradient } from '@/components/ui/text-gradient';
import { Text } from '@/components/ui/text';
import { Map } from '../';

import styles from './ShipArrangement.module.scss';
import { useGameMessage, usePending } from '../../hooks';
import { generateShipsField } from './shipGenerator';
import { convertShipsToField } from '../../utils';
import { useAccount } from '@gear-js/react-hooks';
import { useFetchVoucher } from '@/app/hooks/useFetchVoucher';
import { useCheckBalance } from '@/features/wallet/hooks';

export default function ShipArrangement() {
  const { account } = useAccount();
  const { isVoucher, isLoading, updateBalance } = useFetchVoucher(account?.address);
  const message = useGameMessage();
  const { setPending } = usePending();
  const { checkBalance } = useCheckBalance();

  const [shipLayout, setShipLayout] = useState<string[]>([]);
  const [shipsField, setShipsField] = useState<number[][]>([]);
  const [isLoadingGenerate, setLoadingGenerate] = useState(false);

  const onGenerateRandomLayout = async () => {
    setLoadingGenerate(true);
    const newLayout = await generateShipsField(5, 5);
    const playerShipsLayout = convertShipsToField(newLayout, 5, 5);

    if (newLayout !== null) {
      setShipLayout(playerShipsLayout);
      setShipsField(newLayout);
      setLoadingGenerate(false);
    }
  };

  const onGameStart = async () => {
    const gasLimit = 100000000000;

    await updateBalance();

    if (!isLoading) {
      setPending(true);

      checkBalance(gasLimit, () =>
        message({
          payload: {
            StartGame: {
              ships: shipsField,
            },
          },
          withVoucher: isVoucher,
          gasLimit,
        }),
      );
    }
  };

  return (
    <div className={styles.content}>
      <div className={styles.header}>
        <Heading>
          <TextGradient>Your ships</TextGradient>
        </Heading>
        <div>
          <Text size="lg">Choose a ship placement scheme, and to see a new arrangement, click "Generate"</Text>
        </div>
      </div>
      <div style={{ width: '100%' }}>
        <Map sizeBlock={64} shipStatusArray={shipLayout} />
      </div>
      <div className={styles.buttons}>
        <Button color="dark" text="Generate" onClick={onGenerateRandomLayout} disabled={isLoadingGenerate} />
        <Button text="Continue" onClick={onGameStart} disabled={!shipLayout.length || isLoading} />
      </div>
    </div>
  );
}
