import { usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { useProgram } from '@/app/utils/sails';
import { usePrepareEzTransactionParams } from '@/app/utils/use-make-transaction';

export const useCreateGameMessage = () => {
  const program = useProgram();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'multiple',
    functionName: 'createGame',
  });
  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();

  const createGameMessage = async (name: string) => {
    const { sessionForAccount, ...params } = await prepareEzTransactionParams();
    const { transaction } = await prepareTransactionAsync({
      args: [name, sessionForAccount],
      ...params,
    });
    return transaction;
  };

  return { createGameMessage };
};
