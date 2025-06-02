import { usePrepareProgramTransaction } from '@gear-js/react-hooks';

import { Options, useExecuteWithPending, useSignAndSend } from '@/app/hooks';
import { useProgram } from '@/app/utils';

export const useLeaveGameMessage = () => {
  const program = useProgram();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'galacticExpress',
    functionName: 'leaveGame',
  });
  const { signAndSend } = useSignAndSend();
  const { executeWithPending } = useExecuteWithPending();

  const leaveGameMessage = async (options: Options) =>
    executeWithPending(async () => {
      const { transaction } = await prepareTransactionAsync({
        args: [],
        gasLimit: { increaseGas: 10 },
      });
      return signAndSend(transaction);
    }, options);

  return { leaveGameMessage };
};
