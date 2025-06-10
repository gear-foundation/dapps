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
    const { sessionForAccount: _sessionForAccount, ...params } = await prepareEzTransactionParams();
    const result = await sendTransactionAsync({
      args: [userName, getPkBytes(pk)],
      ...params,
    });
    return result.awaited;
  };

  const { mutateAsync, isPending } = useMutation({
    mutationFn: tx,
    onError: (error) => alert.error(getErrorMessage(error)),
  });

  return { registerMessage: mutateAsync, isPending };
};
