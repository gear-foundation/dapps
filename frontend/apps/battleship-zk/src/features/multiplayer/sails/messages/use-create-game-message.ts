import { usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { usePrepareEzTransactionParams } from '@dapps-frontend/ez-transactions';
import { useProgram } from '@/app/utils/sails';

export const useCreateGameMessage = () => {
  const program = useProgram();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'multiple',
    functionName: 'createGame',
  });
  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();

  const createGameMessage = async (name: string) => {
    const { sessionForAccount, ...params } = await prepareEzTransactionParams(true);
    const { transaction } = await prepareTransactionAsync({
      args: [name, sessionForAccount],
      ...params,
    });
    return transaction;
  };

  return { createGameMessage };
};
