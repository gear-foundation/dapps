import { HexString } from '@gear-js/api';
import { usePrepareProgramTransaction } from '@gear-js/react-hooks';

import { Options, useExecuteWithPending, useSignAndSend } from '@/app/hooks';
import { useProgram } from '@/app/utils';

type Params = {
  value?: bigint;
  adminId: HexString;
  strategyId: HexString;
  name: string;
};

export const useRegisterMessage = () => {
  const program = useProgram();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'syndote',
    functionName: 'register',
  });
  const { signAndSend } = useSignAndSend();
  const { executeWithPending } = useExecuteWithPending();

  const registerMessage = async ({ value, adminId, strategyId, name }: Params, options?: Options) =>
    executeWithPending(async () => {
      const { transaction } = await prepareTransactionAsync({
        args: [adminId, strategyId, name],
        gasLimit: { increaseGas: 10 },
        value,
      });
      await signAndSend(transaction);
    }, options);

  return { registerMessage };
};
