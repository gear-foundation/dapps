import { HexString } from '@gear-js/api';
import { usePrepareProgramTransaction } from '@gear-js/react-hooks';

import { Options, useExecuteWithPending, useSignAndSend } from '@/app/hooks';
import { useProgram } from '@/app/utils';

type Params = {
  adminId: HexString;
};

export const useAddGasToPlayerStrategyMessage = () => {
  const program = useProgram();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'syndote',
    functionName: 'addGasToPlayerStrategy',
  });
  const { signAndSend } = useSignAndSend();
  const { executeWithPending } = useExecuteWithPending();

  const addGasToPlayerStrategyMessage = async ({ adminId }: Params, options?: Options) =>
    executeWithPending(async () => {
      const { transaction } = await prepareTransactionAsync({
        args: [adminId],
        gasLimit: { increaseGas: 10 },
      });
      await signAndSend(transaction);
    }, options);

  return { addGasToPlayerStrategyMessage };
};
