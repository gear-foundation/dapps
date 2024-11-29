import { usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { usePrepareEzTransactionParams } from '@dapps-frontend/ez-transactions';
import { useProgram } from '@/app/utils';
import { Options, useSignAndSend } from '@/app/hooks/use-sign-and-send';

export const useDeletePlayerMessage = () => {
  const program = useProgram();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'varaMan',
    functionName: 'deletePlayer',
  });

  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();
  const { signAndSend } = useSignAndSend();

  const deletePlayerMessage = async (playerId: string, options: Options) => {
    const { sessionForAccount, ...params } = await prepareEzTransactionParams();

    const { transaction } = await prepareTransactionAsync({
      args: [playerId, sessionForAccount],
      ...params,
    });
    signAndSend(transaction, options);
  };

  return { deletePlayerMessage };
};
