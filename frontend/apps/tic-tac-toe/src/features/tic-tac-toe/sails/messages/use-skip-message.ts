import { useSendProgramTransaction } from '@gear-js/react-hooks';
import { usePrepareEzTransactionParams } from 'gear-ez-transactions';

import { useProgram } from '@/app/utils';

export const useSkipMessage = () => {
  const program = useProgram();
  const { sendTransactionAsync } = useSendProgramTransaction({
    program,
    serviceName: 'ticTacToe',
    functionName: 'skip',
  });
  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();

  const skipMessage = async () => {
    const { sessionForAccount, ...params } = await prepareEzTransactionParams();

    return sendTransactionAsync({
      args: [sessionForAccount],
      ...params,
    });
  };

  return { skipMessage };
};
