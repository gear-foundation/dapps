import { HexString } from '@gear-js/api';
import { useAccount, usePrepareProgramTransaction } from '@gear-js/react-hooks';

import { Options, useExecuteWithPending, useSignAndSend } from '@/app/hooks';
import { useProgram } from '@/app/utils';

type Params = {
  accountId: HexString;
};

export const useUnsubscribeMessage = () => {
  const program = useProgram();
  const { account } = useAccount();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'w3Bstreaming',
    functionName: 'unsubscribe',
  });
  const { signAndSend } = useSignAndSend();
  const { executeWithPending } = useExecuteWithPending();

  const unsubscribeMessage = async ({ accountId }: Params, options?: Options) =>
    executeWithPending(async () => {
      if (!account?.decodedAddress) throw new Error('Account is not connected');
      const { transaction } = await prepareTransactionAsync({
        args: [accountId],
        gasLimit: { increaseGas: 10 },
      });
      await signAndSend(transaction);
    }, options);

  return { unsubscribeMessage };
};
