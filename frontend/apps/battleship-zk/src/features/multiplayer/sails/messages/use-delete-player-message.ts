import { usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { usePrepareEzTransactionParams } from 'gear-ez-transactions';
import { useProgram } from '@/app/utils/sails';

export const useDeleteGameMessage = () => {
  const program = useProgram();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'multiple',
    functionName: 'deletePlayer',
  });
  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();

  const deletePlayerMessage = async (playerId: string) => {
    const { sessionForAccount, ...params } = await prepareEzTransactionParams();
    const { transaction } = await prepareTransactionAsync({
      args: [playerId, sessionForAccount],
      ...params,
    });
    return transaction;
  };

  return { deletePlayerMessage };
};
