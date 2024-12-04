import { HexString } from '@gear-js/api';
import { usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { useProgram } from 'app/utils';
import { Options, useExecuteWithPending, useSignAndSend } from 'app/hooks';

type Params = {
  adminId: HexString;
};

export const usePlayMessage = () => {
  const program = useProgram();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'syndote',
    functionName: 'play',
  });
  const { signAndSend } = useSignAndSend();
  const { executeWithPending } = useExecuteWithPending();

  const playMessage = async ({ adminId }: Params, options?: Options) =>
    executeWithPending(async () => {
      const { transaction } = await prepareTransactionAsync({
        args: [adminId],
        gasLimit: BigInt(730000000000),
      });
      await signAndSend(transaction);
    }, options);

  return { playMessage };
};
