import { usePrepareProgramTransaction } from '@gear-js/react-hooks';

import { Options, useSignAndSend } from '@/app/hooks';
import { useProgram } from '@/app/utils';

export const useLeaveGameMessage = () => {
  const program = useProgram();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'galacticExpress',
    functionName: 'leaveGame',
  });
  const { signAndSend } = useSignAndSend();

  const leaveGameMessage = async (options: Options) => {
    const { transaction } = await prepareTransactionAsync({
      args: [],
      gasLimit: { increaseGas: 10 },
    });
    signAndSend(transaction, options);
  };

  return { leaveGameMessage };
};
