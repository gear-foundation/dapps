import { useAlert, usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { useMutation } from '@tanstack/react-query';
import { getErrorMessage } from '@ui/utils';
import { PrepareEzTransactionParamsResult, usePrepareEzTransactionParams } from 'gear-ez-transactions';

import { usePokerProgram } from '@/app/utils';
import { useUserName } from '@/features/game/hooks';
import { useAutoSignless } from '@/features/signless';
import { useKeys } from '@/features/zk/hooks';
import { getPkBytes } from '@/features/zk/utils';

export const useRegisterMessage = () => {
  const program = usePokerProgram();
  const { pk } = useKeys();
  const { userName } = useUserName();
  const alert = useAlert();
  const { executeWithSessionModal } = useAutoSignless();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'poker',
    functionName: 'register',
  });
  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();

  const tx = async () => {
    const { ...ezParams } = await prepareEzTransactionParams();
    const getTransaction = (params?: Partial<PrepareEzTransactionParamsResult>) => {
      const { sessionForAccount, ...rest } = { ...ezParams, ...params };
      const result = prepareTransactionAsync({
        args: [userName, getPkBytes(pk), sessionForAccount],
        ...rest,
      });
      return result;
    };

    await executeWithSessionModal(getTransaction, ezParams.sessionForAccount);
  };

  const { mutateAsync, isPending } = useMutation({
    mutationFn: tx,
    onError: (error) => {
      if (error.message?.includes('Actor id must be exist')) {
        alert.error('Low pts balance. Claim your free PTS');
        return;
      }

      if (error.message?.includes('Low pts balance')) {
        alert.error('Low pts balance');
        return;
      }

      alert.error(getErrorMessage(error));
    },
  });

  return { registerMessage: mutateAsync, isPending };
};
