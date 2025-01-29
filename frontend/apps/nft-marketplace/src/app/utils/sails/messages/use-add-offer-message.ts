import { usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { useMarketplaceProgram } from '@/app/utils';
import { Options, useExecuteWithPending, useSignAndSend } from '@/app/hooks';
import { ADDRESS } from '@/consts';

type Params = {
  tokenId: string;
  price: bigint;
  value: bigint;
};

export const useAddOfferMessage = () => {
  const program = useMarketplaceProgram();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'nftMarketplace',
    functionName: 'addOffer',
  });
  const { signAndSend } = useSignAndSend();
  const { executeWithPending } = useExecuteWithPending();

  const addOfferMessage = async ({ tokenId, price, value }: Params, options?: Options) =>
    executeWithPending(async () => {
      const { transaction } = await prepareTransactionAsync({
        args: [ADDRESS.NFT_CONTRACT, null, tokenId, price],
        gasLimit: { increaseGas: 10 },
        value,
      });
      await signAndSend(transaction);
    }, options);

  return { addOfferMessage };
};
