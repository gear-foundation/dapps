import { useAlert, usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { useMutation } from '@tanstack/react-query';
import { getErrorMessage } from '@ui/utils';
import { PrepareEzTransactionParamsResult, usePrepareEzTransactionParams } from 'gear-ez-transactions';

import { usePokerProgram } from '@/app/utils';
import { useAutoSignless } from '@/features/signless';

type Params = {
  action: Action;
};

export const useTurnMessage = (withAlert = true) => {
  const program = usePokerProgram();
  const alert = useAlert();
  const { executeWithSessionModal } = useAutoSignless();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'poker',
    functionName: 'turn',
  });
  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();

  const tx = async ({ action }: Params) => {
    const { ...ezParams } = await prepareEzTransactionParams();
    const getTransaction = (params?: Partial<PrepareEzTransactionParamsResult>) => {
      const { sessionForAccount, ...rest } = { ...ezParams, ...params };
      const result = prepareTransactionAsync({
        args: [action, sessionForAccount],
        ...rest,
      });
      return result;
    };

    await executeWithSessionModal(getTransaction, ezParams.sessionForAccount);
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
