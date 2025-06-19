import { useAlert, useSendProgramTransaction } from '@gear-js/react-hooks';
import { useMutation } from '@tanstack/react-query';
import { getErrorMessage } from '@ui/utils';
import { usePrepareEzTransactionParams } from 'gear-ez-transactions';

import { usePokerFactoryProgram } from '@/app/utils';

type Params = {
  config: LobbyConfig;
  pk: PublicKey;
};

export const useCreateLobbyMessage = () => {
  const program = usePokerFactoryProgram();
  const alert = useAlert();
  const { sendTransactionAsync } = useSendProgramTransaction({
    program,
    serviceName: 'pokerFactory',
    functionName: 'createLobby',
  });
  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();

  const tx = async ({ config, pk }: Params) => {
    const { sessionForAccount: _sessionForAccount, ...params } = await prepareEzTransactionParams();
    const result = await sendTransactionAsync({ args: [config, pk], ...params, gasLimit: undefined });
    return result.awaited;
  };

  const { mutateAsync: createLobbyMessage, isPending } = useMutation({
    mutationFn: tx,
    onError: (error) => {
      if (error.message.includes('Actor id must be exist')) {
        alert.error('Low pts balance. Claim your free PTS');
        return;
      }

      if (error.message.includes('Low pts balance')) {
        alert.error('Low pts balance');
        return;
      }

      alert.error(getErrorMessage(error));
    },
  });

  return { createLobbyMessage, isPending };
};
