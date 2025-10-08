import { usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { usePrepareEzTransactionParams } from 'gear-ez-transactions';

import { useProgram } from '@/app/utils';

export const useTurnMessage = () => {
  const program = useProgram();
  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'ticTacToe',
    functionName: 'turn',
  });

  const turnMessage = async (step: number) => {
    const { sessionForAccount, ...params } = await prepareEzTransactionParams();
    const { transaction } = await prepareTransactionAsync({ args: [step, sessionForAccount], ...params });

    await transaction.signAndSend();
  };

  return { turnMessage };
};
