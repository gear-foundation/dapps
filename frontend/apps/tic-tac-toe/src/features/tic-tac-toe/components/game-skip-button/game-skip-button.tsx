import { Button } from '@/components/ui/button';
import { useGameMessage, useHandleCalculateGas, useSubscriptionOnGameMessage } from '../../hooks';
import { useEffect, useState } from 'react';
import { useAccount, useAlert } from '@gear-js/react-hooks';
import { ADDRESS } from '../../consts';
import { withoutCommas } from '@/app/utils';
import { ProgramMetadata } from '@gear-js/api';
import { useSignlessTransactions } from '@dapps-frontend/signless-transactions';
import { useGaslessTransactions } from '@dapps-frontend/gasless-transactions';
import { useCheckBalance } from '@dapps-frontend/hooks';

type Props = {
  meta: ProgramMetadata;
};

export function GameSkipButton({ meta }: Props) {
  const calculateGas = useHandleCalculateGas(ADDRESS.GAME, meta);
  const message = useGameMessage(meta);
  const alert = useAlert();
  const { account } = useAccount();

  const signless = useSignlessTransactions();
  const gasless = useGaslessTransactions();

  const { checkBalance } = useCheckBalance({
    signlessPairVoucherId: signless.voucher?.id,
    gaslessVoucherId: gasless.voucherId,
  });

  const [isLoading, setIsLoading] = useState<boolean>(false);
  const { subscribe, unsubscribe, isOpened } = useSubscriptionOnGameMessage(meta);

  useEffect(() => {
    setIsLoading(isOpened);
  }, [isOpened]);

  const onError = () => {
    setIsLoading(false);
    unsubscribe();
  };

  const onSuccess = () => {
    setIsLoading(false);
  };

  const onNextTurn = () => {
    if (!meta || !account || !ADDRESS.GAME) {
      return;
    }

    const payload = { Skip: {} };
    setIsLoading(true);

    calculateGas(payload)
      .then((res) => res.toHuman())
      .then(({ min_limit }) => {
        const minLimit = withoutCommas(min_limit as string);
        const gasLimit = Math.floor(Number(minLimit) + Number(minLimit) * 0.2);

        subscribe();
        checkBalance(
          gasLimit,
          () => {
            message({
              payload,
              gasLimit,
              onError,
              onSuccess,
            });
          },
          onError,
        );
      })
      .catch((error) => {
        onError();
        console.log(error);
        alert.error('Gas calculation error');
      });
  };

  return (
    <Button onClick={onNextTurn} isLoading={isLoading} variant="black">
      Skip
    </Button>
  );
}
