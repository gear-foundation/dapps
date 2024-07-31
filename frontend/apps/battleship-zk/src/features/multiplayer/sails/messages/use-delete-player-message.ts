import { useProgram } from '@/app/utils/sails';
import { useMakeTransaction } from '@/app/utils/use-make-transaction';

export const useDeleteGameMessage = () => {
  const gasLimit = 250_000_000_000n;
  const makeTransaction = useMakeTransaction();
  const program = useProgram();

  const deletePlayerMessage = async (playerId: string) => {
    if (!program) throw new Error('program does not found');

    const transaction = await makeTransaction(program.multiple.deletePlayer(playerId, null));

    return await transaction.withGas(gasLimit);
  };

  return { deletePlayerMessage };
};
