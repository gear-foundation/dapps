import { useProgram } from '@/app/utils/sails';
import { useMakeTransaction } from '@/app/utils/use-make-transaction';

export const useJoinGameMessage = () => {
  const gasLimit = 250_000_000_000n;
  const makeTransaction = useMakeTransaction();
  const program = useProgram();

  const joinGameMessage = async (game_id: string, name: string) => {
    if (!program) throw new Error('program does not found');

    const transaction = await makeTransaction(program.multiple.joinGame(game_id, name, null));

    return await transaction.withGas(gasLimit);
  };

  return { joinGameMessage };
};
