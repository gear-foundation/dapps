import { useProgram } from '@/app/utils/sails';
import { useMakeTransaction } from '@/app/utils/use-make-transaction';

export const useCancelGameMessage = () => {
  const gasLimit = 250_000_000_000n;
  const makeTransaction = useMakeTransaction();
  const program = useProgram();

  const cancelGameMessage = async () => {
    if (!program) throw new Error('program does not found');

    const transaction = await makeTransaction(program.multiple.cancelGame(null));

    return await transaction.withGas(gasLimit);
  };

  return { cancelGameMessage };
};
