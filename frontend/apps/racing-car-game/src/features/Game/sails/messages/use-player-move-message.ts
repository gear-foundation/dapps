import { useSendProgramTransaction } from '@gear-js/react-hooks';
import { usePrepareEzTransactionParams } from 'gear-ez-transactions';

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

  const playerMoveMessage = async (strategyMove: StrategyAction, { onError }: Options) => {
    try {
      const { sessionForAccount, ...params } = await prepareEzTransactionParams();
      const { result } = await sendTransactionAsync({
        args: [strategyMove, sessionForAccount],
        ...params,
      });
      return result.response();
    } catch (e) {
      onError?.();
      console.error(e);
    }
  };

  return { playerMoveMessage };
};
