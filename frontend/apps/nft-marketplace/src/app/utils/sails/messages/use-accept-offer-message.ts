import { usePrepareProgramTransaction } from '@gear-js/react-hooks';

import { Options, useExecuteWithPending, useSignAndSend } from '@/app/hooks';
import { useMarketplaceProgram } from '@/app/utils';
import { ENV } from '@/consts';

type Params = {
  tokenId: string;
  price: bigint;
};

export const useAcceptOfferMessage = () => {
  const program = useMarketplaceProgram();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'nftMarketplace',
    functionName: 'acceptOffer',
  });
  const { signAndSend } = useSignAndSend();
  const { executeWithPending } = useExecuteWithPending();

  const acceptOfferMessage = async ({ tokenId, price }: Params, options?: Options) =>
    executeWithPending(async () => {
      const { transaction } = await prepareTransactionAsync({
        args: [ENV.NFT_CONTRACT, null, tokenId, price],
        gasLimit: { increaseGas: 10 },
      });
      await signAndSend(transaction);
    }, options);

  return { acceptOfferMessage };
};
