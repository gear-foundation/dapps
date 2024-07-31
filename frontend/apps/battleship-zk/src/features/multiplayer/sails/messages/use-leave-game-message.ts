import { useProgram } from '@/app/utils/sails';
import { useMakeTransaction } from '@/app/utils/use-make-transaction';

export const useLeaveGameMessage = () => {
  const gasLimit = 250_000_000_000n;
  const makeTransaction = useMakeTransaction();
  const program = useProgram();

  const leaveGameMessage = async () => {
    if (!program) throw new Error('program does not found');

    const transaction = await makeTransaction(program.multiple.leaveGame(null));

    return await transaction.withGas(gasLimit);
  };

  return { leaveGameMessage };
};
