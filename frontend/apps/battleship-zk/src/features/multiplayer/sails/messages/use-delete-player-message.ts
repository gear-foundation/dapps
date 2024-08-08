import { usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { useProgram } from '@/app/utils/sails';
import { usePrepareEzTransactionParams } from '@/app/utils/use-make-transaction';

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
