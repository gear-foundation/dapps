import { program } from '@/app/utils/sails';
import { useMakeTransaction } from '@/app/utils/use-make-transaction';

export const useMakeMoveMessage = () => {
  const gasLimit = 250_000_000_000n;
  const makeTransaction = useMakeTransaction();

  const makeMoveMessage = async (game_id: string, step: number) => {
    const transaction = await makeTransaction(program.multiple.makeMove(game_id, step, null));

    return await transaction.withGas(gasLimit);
  };

  return { makeMoveMessage };
};
