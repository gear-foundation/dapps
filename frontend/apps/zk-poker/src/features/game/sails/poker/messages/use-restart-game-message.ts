import { useAlert, useSendProgramTransaction } from '@gear-js/react-hooks';
import { useMutation } from '@tanstack/react-query';
import { getErrorMessage } from '@ui/utils';
import { usePrepareEzTransactionParams } from 'gear-ez-transactions';

import { usePokerProgram } from '@/app/utils';

export const useRestartGameMessage = () => {
  const program = usePokerProgram();
  const alert = useAlert();
  const { sendTransactionAsync } = useSendProgramTransaction({
    program,
    serviceName: 'poker',
    functionName: 'restartGame',
  });
  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();

  const tx = async () => {
    const { sessionForAccount, ...params } = await prepareEzTransactionParams();
    const result = await sendTransactionAsync({ args: [sessionForAccount], ...params });
    return result.awaited;
  };

  const { mutateAsync, isPending } = useMutation({
    mutationFn: tx,
    onError: (error) => alert.error(getErrorMessage(error)),
  });

  return { restartGameMessage: mutateAsync, isPending };
};
