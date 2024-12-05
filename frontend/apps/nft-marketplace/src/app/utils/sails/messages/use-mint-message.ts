import { useAccount, usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { useNftProgram } from 'app/utils';
import { Options, useExecuteWithPending, useSignAndSend } from 'app/hooks';
import { TokenMetadata } from '../nft';

type Params = TokenMetadata;

export const useMintMessage = () => {
  const program = useNftProgram();
  const { account } = useAccount();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'vnft',
    functionName: 'mint',
  });
  const { signAndSend } = useSignAndSend();
  const { executeWithPending } = useExecuteWithPending();

  const mintMessage = async (tokenMetadata: Params, options?: Options) =>
    executeWithPending(async () => {
      if (!account?.decodedAddress) throw 'account is not connected';
      const { transaction } = await prepareTransactionAsync({
        args: [account.decodedAddress, tokenMetadata],
        gasLimit: { increaseGas: 10 },
      });
      await signAndSend(transaction);
    }, options);

  return { mintMessage };
};
