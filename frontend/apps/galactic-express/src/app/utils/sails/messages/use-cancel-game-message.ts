import { usePrepareProgramTransaction } from '@gear-js/react-hooks';

import { Options, useExecuteWithPending, useSignAndSend } from '@/app/hooks';
import { useProgram } from '@/app/utils';

export const useCancelGameMessage = () => {
  const program = useProgram();
  const { executeWithPending } = useExecuteWithPending();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'galacticExpress',
    functionName: 'cancelGame',
  });
  const { signAndSend } = useSignAndSend();

  const cancelGameMessage = async (options: Options) =>
    executeWithPending(async () => {
      const { transaction } = await prepareTransactionAsync({
        args: [],
        gasLimit: { increaseGas: 10 },
      });
      return signAndSend(transaction);
    }, options);

  return { cancelGameMessage };
};
