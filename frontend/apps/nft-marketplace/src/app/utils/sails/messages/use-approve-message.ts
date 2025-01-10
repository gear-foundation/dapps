import { useAccount, usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { useNftProgram } from 'app/utils';
import { Options, useExecuteWithPending, useSignAndSend } from 'app/hooks';
import { ADDRESS } from 'consts';

type Params = {
  tokenId: string;
};

export const useApproveMessage = () => {
  const program = useNftProgram();
  const { account } = useAccount();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'vnft',
    functionName: 'approve',
  });
  const { signAndSend } = useSignAndSend();
  const { executeWithPending } = useExecuteWithPending();

  const approveMessage = async ({ tokenId }: Params, options?: Options) =>
    executeWithPending(async () => {
      if (!account?.decodedAddress) throw 'account is not connected';
      const { transaction } = await prepareTransactionAsync({
        args: [ADDRESS.MARKETPLACE_CONTRACT, tokenId],
        gasLimit: { increaseGas: 10 },
      });
      await signAndSend(transaction);
    }, options);

  return { approveMessage };
};
