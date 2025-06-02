import { usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { usePrepareEzTransactionParams } from 'gear-ez-transactions';

import { Options, useSignAndSend } from '@/app/hooks/use-sign-and-send';
import { useProgram } from '@/app/utils';

export const useDeletePlayerMessage = () => {
  const program = useProgram();

  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'battle',
    functionName: 'deletePlayer',
  });

  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();
  const { signAndSend } = useSignAndSend();

  const deletePlayerMessage = async (playerId: string, options?: Options) => {
    const { sessionForAccount, ...params } = await prepareEzTransactionParams();

    const { transaction } = await prepareTransactionAsync({
      args: [playerId, sessionForAccount],
      ...params,
    });

    signAndSend(transaction, options);
  };

  return { deletePlayerMessage };
};
