import { useSendProgramTransaction } from '@gear-js/react-hooks';
import { usePrepareEzTransactionParams } from 'gear-ez-transactions';

import { useProgram } from '@/app/utils';

export const useStartGameMessage = () => {
  const program = useProgram();
  const { sendTransactionAsync } = useSendProgramTransaction({
    program,
    serviceName: 'ticTacToe',
    functionName: 'startGame',
  });
  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();

  const startGameMessage = async () => {
    const { sessionForAccount, ...params } = await prepareEzTransactionParams();

    return sendTransactionAsync({
      args: [sessionForAccount],
      ...params,
    });
  };

  return { startGameMessage };
};
