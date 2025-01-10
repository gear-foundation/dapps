import { HexString } from '@gear-js/api';
import { useAccount, usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { useProgram } from 'app/utils';
import { Options, useExecuteWithPending, useSignAndSend } from 'app/hooks';

type Params = {
  accountId: HexString;
};

export const useSubscribeMessage = () => {
  const program = useProgram();
  const { account } = useAccount();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'w3Bstreaming',
    functionName: 'subscribe',
  });
  const { signAndSend } = useSignAndSend();
  const { executeWithPending } = useExecuteWithPending();

  const subscribeMessage = async ({ accountId }: Params, options?: Options) =>
    executeWithPending(async () => {
      if (!account?.decodedAddress) throw 'account is not connected';
      const { transaction } = await prepareTransactionAsync({
        args: [accountId],
        gasLimit: { increaseGas: 10 },
      });
      await signAndSend(transaction);
    }, options);

  return { subscribeMessage };
};
