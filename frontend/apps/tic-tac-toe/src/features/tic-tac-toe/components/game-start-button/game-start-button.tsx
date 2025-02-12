import { useAccount, useAlert } from '@gear-js/react-hooks';
import { useGaslessTransactions } from 'gear-ez-transactions';
import { useAtom } from 'jotai';

import { BaseComponentProps } from '@/app/types';
import { Button } from '@/components/ui/button';

import { useStartGameMessage, useEventGameStartedSubscription } from '../../sails';
import { stateChangeLoadingAtom } from '../../store';

type GameStartButtonProps = BaseComponentProps;

export function GameStartButton({ children }: GameStartButtonProps) {
  const { startGameMessage } = useStartGameMessage();
  const { account } = useAccount();
  const alert = useAlert();
  const gasless = useGaslessTransactions();
  const [isLoading, setIsLoading] = useAtom(stateChangeLoadingAtom);

  useEventGameStartedSubscription();

  const onGameStart = async () => {
    if (!account) {
      return;
    }

    setIsLoading(true);
    try {
      await startGameMessage();
    } catch (error) {
      console.log(error);
      alert.error((error instanceof Error && error.message) || 'Game start error');
      setIsLoading(false);
    }
  };

  return (
    <Button onClick={onGameStart} isLoading={isLoading || !account || gasless.isLoading}>
      {children}
    </Button>
  );
}
