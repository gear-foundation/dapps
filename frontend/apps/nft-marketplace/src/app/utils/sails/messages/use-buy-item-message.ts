import { usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { useMarketplaceProgram } from 'app/utils';
import { Options, useExecuteWithPending, useSignAndSend } from 'app/hooks';
import { ADDRESS } from 'consts';

type Params = {
  tokenId: string;
  value: bigint;
};

export const useBuyItemMessage = () => {
  const program = useMarketplaceProgram();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'nftMarketplace',
    functionName: 'buyItem',
  });
  const { signAndSend } = useSignAndSend();
  const { executeWithPending } = useExecuteWithPending();

  const buyItemMessage = async ({ tokenId, value }: Params, options?: Options) =>
    executeWithPending(async () => {
      const { transaction } = await prepareTransactionAsync({
        args: [ADDRESS.NFT_CONTRACT, tokenId],
        gasLimit: { increaseGas: 10 },
        value,
      });
      await signAndSend(transaction);
    }, options);

  return { buyItemMessage };
};
