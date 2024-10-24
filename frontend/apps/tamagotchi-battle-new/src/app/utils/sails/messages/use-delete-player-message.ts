import { usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { useProgram } from '@/app/utils';
import { Options, useSignAndSend } from '@/app/hooks/use-sign-and-send';

export const useDeletePlayerMessage = () => {
  const program = useProgram();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'battle',
    functionName: 'deletePlayer',
  });
  const { signAndSend } = useSignAndSend();

  const deletePlayerMessage = async (playerId: string, options?: Options) => {
    const { transaction } = await prepareTransactionAsync({
      args: [playerId],
      gasLimit: { increaseGas: 10 },
    });
    signAndSend(transaction, options);
  };

  return { deletePlayerMessage };
};
