import { useAlert, useSendProgramTransaction } from '@gear-js/react-hooks';
import { useMutation } from '@tanstack/react-query';
import { getErrorMessage } from '@ui/utils';
import { usePrepareEzTransactionParams } from 'gear-ez-transactions';

import { useDnsProgramIds } from '@dapps-frontend/hooks';

import { usePokerProgram } from '@/app/utils';

export const useKillMessage = () => {
  const program = usePokerProgram();
  const alert = useAlert();
  const { sendTransactionAsync } = useSendProgramTransaction({
    program,
    serviceName: 'poker',
    functionName: 'kill',
  });
  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();
  const { pokerFactoryProgramId } = useDnsProgramIds<'pokerFactoryProgramId'>();

  const tx = async () => {
    const { sessionForAccount: _sessionForAccount, ...params } = await prepareEzTransactionParams();
    const result = await sendTransactionAsync({
      args: [pokerFactoryProgramId],
      ...params,
    });
    return result.awaited;
  };

  const { mutateAsync, isPending } = useMutation({
    mutationFn: tx,
    onError: (error) => alert.error(getErrorMessage(error)),
  });

  return { killMessage: mutateAsync, isPending };
};
