import { Button } from '@/components/ui/button';
import { useGameMessage, useHandleCalculateGas, useSubscriptionOnGameMessage } from '../../hooks';
import { useEffect, useState } from 'react';
import { useCheckBalance } from '@/app/hooks';
import { useAccount, useAlert } from '@gear-js/react-hooks';
import { ADDRESS } from '../../consts';
import { withoutCommas } from '@/app/utils';
import { ProgramMetadata } from '@gear-js/api';

type Props = {
  meta: ProgramMetadata;
};

export function GameSkipButton({ meta }: Props) {
  const calculateGas = useHandleCalculateGas(ADDRESS.GAME, meta);
  const message = useGameMessage(meta);
  const alert = useAlert();
  const { account } = useAccount();
  const { checkBalance } = useCheckBalance();
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
    console.log('success on skip');
  };

  const onNextTurn = () => {
    if (!meta || !account || !ADDRESS.GAME) {
      return;
    }

    const payload = { Skip: null };
    setIsLoading(true);

    calculateGas(payload)
      .then((res) => res.toHuman())
      .then(({ min_limit }) => {
        console.log('min_limit================');
        console.log(min_limit);
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
