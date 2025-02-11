import { HexString } from '@gear-js/api';
import { usePrepareProgramTransaction } from '@gear-js/react-hooks';

import { Options, useExecuteWithPending, useSignAndSend } from '@/app/hooks';
import { useProgram } from '@/app/utils';

type Params = {
  adminId: HexString;
};

export const useCancelGameSessionMessage = () => {
  const program = useProgram();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'syndote',
    functionName: 'cancelGameSession',
  });
  const { signAndSend } = useSignAndSend();
  const { executeWithPending } = useExecuteWithPending();

  const cancelGameSessionMessage = async ({ adminId }: Params, options?: Options) =>
    executeWithPending(async () => {
      const { transaction } = await prepareTransactionAsync({
        args: [adminId],
        gasLimit: { increaseGas: 10 },
      });
      await signAndSend(transaction);
    }, options);

  return { cancelGameSessionMessage };
};
