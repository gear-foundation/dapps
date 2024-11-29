import { useSendProgramTransaction } from '@gear-js/react-hooks';
import { usePrepareEzTransactionParams } from '@dapps-frontend/ez-transactions';
import { useProgram } from '@/app/utils';

type Options = {
  onError?: () => void;
};

export const useStartGameMessage = () => {
  const program = useProgram();
  const { sendTransactionAsync } = useSendProgramTransaction({
    program,
    serviceName: 'carRacesService',
    functionName: 'startGame',
  });
  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();

  const startGameMessage = async ({ onError }: Options) => {
    try {
      const { sessionForAccount, ...params } = await prepareEzTransactionParams();
      const { result } = await sendTransactionAsync({
        args: [sessionForAccount],
        ...params,
      });
      return result.response();
    } catch (e) {
      onError?.();
      console.error(e);
    }
  };

  return { startGameMessage };
};
