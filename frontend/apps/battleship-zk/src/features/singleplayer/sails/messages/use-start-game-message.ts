import { program } from '@/app/utils/sails';
import { useMakeTransaction } from '@/app/utils/use-make-transaction';
import { ProofBytes, PublicStartInput } from '@/features/game/assets/lib/lib';

export const useStartGameMessage = () => {
  const gasLimit = 250_000_000_000n;
  const makeTransaction = useMakeTransaction();

  const startGameMessage = async (proof: ProofBytes, public_input: PublicStartInput) => {
    const transaction = await makeTransaction(program.single.startSingleGame(proof, public_input, null));

    return await transaction.withGas(gasLimit);
  };

  return { startGameMessage };
};
