import { usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { usePrepareEzTransactionParams } from 'gear-ez-transactions';

import { useProgram } from '@/app/utils/sails';

export const useCreateGameMessage = () => {
  const program = useProgram();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'multiple',
    functionName: 'createGame',
  });
  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();

  const createGameMessage = async (name: string, value: bigint) => {
    const { sessionForAccount, ...params } = await prepareEzTransactionParams({ sendFromBaseAccount: true });
    const { transaction } = await prepareTransactionAsync({
      args: [name, sessionForAccount],
      ...params,
      value,
    });
    return transaction;
  };

  return { createGameMessage };
};
