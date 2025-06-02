import { usePrepareProgramTransaction } from '@gear-js/react-hooks';

import { Options, useExecuteWithPending, useSignAndSend } from '@/app/hooks';
import { useMarketplaceProgram } from '@/app/utils';
import { ENV } from '@/consts';

type Params = {
  tokenId: string;
  price: bigint;
  value: bigint;
};

export const useAddBidMessage = () => {
  const program = useMarketplaceProgram();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'nftMarketplace',
    functionName: 'addBid',
  });
  const { signAndSend } = useSignAndSend();
  const { executeWithPending } = useExecuteWithPending();

  const addBidMessage = async ({ tokenId, price, value }: Params, options?: Options) =>
    executeWithPending(async () => {
      const { transaction } = await prepareTransactionAsync({
        args: [ENV.NFT_CONTRACT, tokenId, price],
        gasLimit: { increaseGas: 10 },
        value,
      });
      await signAndSend(transaction);
    }, options);

  return { addBidMessage };
};
