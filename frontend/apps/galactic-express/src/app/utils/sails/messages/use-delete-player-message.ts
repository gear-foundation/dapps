import { usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { useProgram } from 'app/utils';
import { Options, useSignAndSend } from 'app/hooks';
import { HexString } from '@gear-js/api';

type Params = {
  playerId: HexString;
};

export const useDeletePlayerMessage = () => {
  const program = useProgram();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'galacticExpress',
    functionName: 'deletePlayer',
  });
  const { signAndSend } = useSignAndSend();

  const deletePlayerMessage = async ({ playerId }: Params, options?: Options) => {
    const { transaction } = await prepareTransactionAsync({
      args: [playerId],
      gasLimit: { increaseGas: 10 },
    });
    signAndSend(transaction, options);
  };

  return { deletePlayerMessage };
};
