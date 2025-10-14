import { useAlert, useSendProgramTransaction } from '@gear-js/react-hooks';
import { useMutation } from '@tanstack/react-query';
import { getErrorMessage } from '@ui/utils';
import { usePrepareEzTransactionParams } from 'gear-ez-transactions';

import { usePokerProgram } from '@/app/utils';

type Params = {
  action: Action;
};

export const useTurnMessage = (withAlert = true) => {
  const program = usePokerProgram();
  const alert = useAlert();
  const { sendTransactionAsync } = useSendProgramTransaction({
    program,
    serviceName: 'poker',
    functionName: 'turn',
  });
  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();

  const tx = async ({ action }: Params) => {
    const { sessionForAccount, ...params } = await prepareEzTransactionParams();
    const result = await sendTransactionAsync({ args: [action, sessionForAccount], ...params });
    return result;
  };

  const { mutateAsync, isPending } = useMutation({
    mutationFn: tx,
    onError: (error) => {
      if (withAlert) {
        alert.error(getErrorMessage(error));
      }
    },
  });

  return { turnMessage: mutateAsync, isPending };
};
