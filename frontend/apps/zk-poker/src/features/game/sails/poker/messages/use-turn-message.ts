import { useAlert, useSendProgramTransaction } from '@gear-js/react-hooks';
import { useMutation } from '@tanstack/react-query';
import { getErrorMessage } from '@ui/utils';
import { usePrepareEzTransactionParams } from 'gear-ez-transactions';

import { usePokerProgram } from '@/app/utils';

type Params = {
  action: Action;
};

export const useTurnMessage = () => {
  const program = usePokerProgram();
  const alert = useAlert();
  const { sendTransactionAsync } = useSendProgramTransaction({
    program,
    serviceName: 'poker',
    functionName: 'turn',
  });
  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();

  const tx = async ({ action }: Params) => {
    const { sessionForAccount: _sessionForAccount, ...params } = await prepareEzTransactionParams();
    const result = await sendTransactionAsync({ args: [action], ...params });
    return result.awaited;
  };

  const { mutateAsync, isPending } = useMutation({
    mutationFn: tx,
    onError: (error) => alert.error(getErrorMessage(error)),
  });

  return { turnMessage: mutateAsync, isPending };
};
