import { Button } from '@/components/ui/button';
import { useGameMessage, useHandleCalculateGas, useSubscriptionOnGameMessage } from '../../hooks';
import { useEffect } from 'react';
import { BaseComponentProps } from '@/app/types';
import { useCheckBalance } from '@/app/hooks';
import { useAccount, useAlert } from '@gear-js/react-hooks';
import { ADDRESS } from '../../consts';
import { withoutCommas } from '@/app/utils';
import { ProgramMetadata } from '@gear-js/api';
import { useAtom } from 'jotai';
import { stateChangeLoadingAtom } from '../../store';

type GameStartButtonProps = BaseComponentProps & {
  meta: ProgramMetadata;
};

export function GameStartButton({ children, meta }: GameStartButtonProps) {
  const calculateGas = useHandleCalculateGas(ADDRESS.GAME, meta);
  const message = useGameMessage(meta);
  const { account } = useAccount();
  const alert = useAlert();
  const { checkBalance } = useCheckBalance();
  const [isLoading, setIsLoading] = useAtom(stateChangeLoadingAtom);
  const { subscribe, unsubscribe, isOpened } = useSubscriptionOnGameMessage(meta);

  useEffect(() => {
    console.log({ isOpened });
    setIsLoading(isOpened);
  }, [isOpened]);

  const onError = () => {
    setIsLoading(false);
    unsubscribe();
  };
  const onSuccess = () => {
    console.log('success on start');
  };

  const onGameStart = () => {
    if (!meta || !account || !ADDRESS.GAME) {
      return;
    }
    const payload = { StartGame: null };
    setIsLoading(true);

    calculateGas(payload)
      .then((res) => res.toHuman())
      .then(({ min_limit }) => {
        const minLimit = withoutCommas(min_limit as string);
        const gasLimit = Math.floor(Number(minLimit) + Number(minLimit) * 0.2);
        console.log('min_limit================');
        console.log(min_limit);
        console.log(gasLimit);

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
    <Button onClick={onGameStart} isLoading={isLoading || !meta || !ADDRESS.GAME || !account}>
      {children}
    </Button>
  );
}
