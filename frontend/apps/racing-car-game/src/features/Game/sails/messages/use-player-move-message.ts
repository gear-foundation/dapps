import { useAlert, useSendProgramTransaction } from '@gear-js/react-hooks';
import { usePrepareEzTransactionParams } from 'gear-ez-transactions';

import { getErrorMessage } from '@dapps-frontend/ui';

import { StrategyAction, useProgram } from '@/app/utils';

type Options = {
  onError?: () => void;
};

export const usePlayerMoveMessage = () => {
  const program = useProgram();
  const { sendTransactionAsync } = useSendProgramTransaction({
    program,
    serviceName: 'carRacesService',
    functionName: 'playerMove',
  });
  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();
  const alert = useAlert();

  const playerMoveMessage = async (strategyMove: StrategyAction, { onError }: Options) => {
    try {
      const { sessionForAccount, ...params } = await prepareEzTransactionParams();
      const { result } = await sendTransactionAsync({
        args: [strategyMove, sessionForAccount],
        ...params,
      });
      return result.response();
    } catch (error) {
      onError?.();
      alert.error(getErrorMessage(error));
      console.error(error);
    }
  };

  return { playerMoveMessage };
};
