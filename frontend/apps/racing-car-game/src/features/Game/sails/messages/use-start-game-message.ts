import { useAlert, useSendProgramTransaction } from '@gear-js/react-hooks';
import { usePrepareEzTransactionParams } from 'gear-ez-transactions';

import { getErrorMessage } from '@dapps-frontend/ui';

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
  const alert = useAlert();

  const startGameMessage = async ({ onError }: Options) => {
    try {
      const { sessionForAccount, ...params } = await prepareEzTransactionParams();
      const { result } = await sendTransactionAsync({
        args: [sessionForAccount],
        ...params,
      });
      return result.response();
    } catch (error) {
      onError?.();
      alert.error(getErrorMessage(error));
      console.error(error);
    }
  };

  return { startGameMessage };
};
