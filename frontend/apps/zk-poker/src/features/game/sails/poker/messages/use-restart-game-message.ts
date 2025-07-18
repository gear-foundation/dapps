import { useAlert, usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { useMutation } from '@tanstack/react-query';
import { getErrorMessage } from '@ui/utils';
import { usePrepareEzTransactionParams } from 'gear-ez-transactions';

import { usePokerProgram } from '@/app/utils';
import { useAutoSignless } from '@/features/signless';

export const useRestartGameMessage = () => {
  const program = usePokerProgram();
  const alert = useAlert();
  const { executeWithSessionModal } = useAutoSignless();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'poker',
    functionName: 'restartGame',
  });
  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();

  const tx = async () => {
    const { sessionForAccount, ...params } = await prepareEzTransactionParams();
    const { transaction } = await prepareTransactionAsync({ args: [sessionForAccount], ...params });

    await executeWithSessionModal(transaction, sessionForAccount);
  };

  const { mutateAsync, isPending } = useMutation({
    mutationFn: tx,
    onError: (error) => alert.error(getErrorMessage(error)),
  });

  return { restartGameMessage: mutateAsync, isPending };
};
