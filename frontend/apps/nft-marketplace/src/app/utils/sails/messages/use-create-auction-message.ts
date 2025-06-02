import { usePrepareProgramTransaction } from '@gear-js/react-hooks';

import { Options, useExecuteWithPending, useSignAndSend } from '@/app/hooks';
import { useMarketplaceProgram } from '@/app/utils';
import { ENV } from '@/consts';

type Params = {
  tokenId: string;
  minPrice: bigint;
  duration: bigint;
};

export const useCreateAuctionMessage = () => {
  const program = useMarketplaceProgram();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'nftMarketplace',
    functionName: 'createAuction',
  });
  const { signAndSend } = useSignAndSend();
  const { executeWithPending } = useExecuteWithPending();

  const createAuctionMessage = async ({ tokenId, minPrice, duration }: Params, options?: Options) =>
    executeWithPending(async () => {
      const { transaction } = await prepareTransactionAsync({
        args: [ENV.NFT_CONTRACT, null, tokenId, minPrice, duration],
        gasLimit: { increaseGas: 10 },
      });
      await signAndSend(transaction);
    }, options);

  return { createAuctionMessage };
};
