import { useAlert, useSendProgramTransaction } from '@gear-js/react-hooks';
import { useMutation } from '@tanstack/react-query';
import { getErrorMessage } from '@ui/utils';
import { usePrepareEzTransactionParams } from 'gear-ez-transactions';

import { usePokerProgram } from '@/app/utils';
import { useUserName } from '@/features/game/hooks';
import { useKeys } from '@/features/zk/hooks';
import { getPkBytes } from '@/features/zk/utils';

export const useRegisterMessage = () => {
  const program = usePokerProgram();
  const { pk } = useKeys();
  const { userName } = useUserName();
  const alert = useAlert();
  const { sendTransactionAsync } = useSendProgramTransaction({
    program,
    serviceName: 'poker',
    functionName: 'register',
  });
  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();

  const tx = async () => {
    const { sessionForAccount, ...params } = await prepareEzTransactionParams();
    const result = await sendTransactionAsync({
      args: [userName, getPkBytes(pk), sessionForAccount],
      ...params,
    });
    return result.awaited;
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
