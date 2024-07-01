import { program } from '@/app/utils/sails';
import { useMakeTransaction } from '@/app/utils/use-make-transaction';

export const useCreateGameMessage = () => {
  const gasLimit = 250_000_000_000n;
  const makeTransaction = useMakeTransaction();

  const createGameMessage = async (name: string) => {
    const transaction = await makeTransaction(program.multiple.createGame(name, null));

    return await transaction.withGas(gasLimit);
  };

  return { createGameMessage };
};
