import { program } from '@/app/utils/sails';
import { useMakeTransaction } from '@/app/utils/use-make-transaction';

export const useCancelGameMessage = () => {
  const gasLimit = 250_000_000_000n;
  const makeTransaction = useMakeTransaction();

  const cancelGameMessage = async () => {
    const transaction = await makeTransaction(program.multiple.cancelGame(null));

    return await transaction.withGas(gasLimit);
  };

  return { cancelGameMessage };
};
