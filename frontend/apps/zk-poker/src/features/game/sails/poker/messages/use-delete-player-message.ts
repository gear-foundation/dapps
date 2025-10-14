import { useAlert, useSendProgramTransaction } from '@gear-js/react-hooks';
import { useMutation } from '@tanstack/react-query';
import { getErrorMessage } from '@ui/utils';
import { usePrepareEzTransactionParams } from 'gear-ez-transactions';
import { ActorId } from 'sails-js';

import { usePokerProgram } from '@/app/utils';

export const useDeletePlayerMessage = () => {
  const program = usePokerProgram();
  const alert = useAlert();
  const { sendTransactionAsync } = useSendProgramTransaction({
    program,
    serviceName: 'poker',
    functionName: 'deletePlayer',
  });
  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();

  const tx = async (playerId: ActorId) => {
    const { sessionForAccount, ...params } = await prepareEzTransactionParams({ isAutoSignlessEnabled: true });
    const result = await sendTransactionAsync({ args: [playerId, sessionForAccount], ...params });
    return result;
  };

  const { mutateAsync, isPending } = useMutation({
    mutationFn: tx,
    onError: (error) => alert.error(getErrorMessage(error)),
  });

  return { deletePlayerMessage: mutateAsync, isPending };
};
