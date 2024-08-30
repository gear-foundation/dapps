import { Button } from '@/components/ui/button';
import { BaseComponentProps } from '@/app/types';
import { useAccount, useAlert } from '@gear-js/react-hooks';
import { useGaslessTransactions } from '@dapps-frontend/ez-transactions';
import { useAtom } from 'jotai';
import { stateChangeLoadingAtom } from '../../store';
import { useStartGameMessage, useEventGameStartedSubscription } from '../../sails';

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
