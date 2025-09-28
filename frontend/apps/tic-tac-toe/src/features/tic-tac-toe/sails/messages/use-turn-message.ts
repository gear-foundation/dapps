import { useSendProgramTransaction } from '@gear-js/react-hooks';
import { usePrepareEzTransactionParams } from 'gear-ez-transactions';

import { useProgram } from '@/app/utils';

export const useTurnMessage = () => {
  const program = useProgram();
  const { sendTransactionAsync } = useSendProgramTransaction({
    program,
    serviceName: 'ticTacToe',
    functionName: 'turn',
  });
  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();

  const turnMessage = async (step: number) => {
    const { sessionForAccount, ...params } = await prepareEzTransactionParams();

    return sendTransactionAsync({
      args: [step, sessionForAccount],
      ...params,
    });
  };

  return { turnMessage };
};
