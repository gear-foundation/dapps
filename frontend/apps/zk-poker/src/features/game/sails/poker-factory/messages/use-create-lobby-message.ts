import { useAccount, useAlert, usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { useMutation } from '@tanstack/react-query';
import { getErrorMessage } from '@ui/utils';
import { usePrepareEzTransactionParams } from 'gear-ez-transactions';

import { usePokerFactoryProgram } from '@/app/utils';

type Params = {
  config: GameConfig;
  pk: ZkPublicKey;
};

export const useCreateLobbyMessage = () => {
  const program = usePokerFactoryProgram();
  const { account } = useAccount();

  const alert = useAlert();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'pokerFactory',
    functionName: 'createLobby',
  });
  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();

  const tx = async ({ config, pk }: Params) => {
    if (!program || !account) return;

    const { sessionForAccount: _, voucherId, account: senderAccount } = await prepareEzTransactionParams();

    const signatureInfo = null;

    const { transaction } = await prepareTransactionAsync({
      args: [config, pk, signatureInfo],
      account: senderAccount,
      value: 1_000_000_000_000n,
      gasLimit: { increaseGas: 5 },
      // gasLimit: 705_000_000_000n,
      voucherId,
    });

    const result = await transaction.signAndSend();
    return result.response();
  };

  const { mutateAsync: createLobbyMessage, isPending } = useMutation({
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

  return { createLobbyMessage, isPending };
};
